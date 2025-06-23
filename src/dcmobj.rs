use dicom::core::Tag;
use dicom::dictionary_std::{tags, uids};
use dicom::object::DefaultDicomObject;
use dicom::pixeldata::Transcode;
use dicom_core::Length;
use dicom_core::header::HasLength;
use dicom_encoding::snafu::Report;
use dicom_object::{FileDicomObject, InMemDicomObject, OpenFileOptions, open_file};
use gdcm_conv::PhotometricInterpretation;
use log::warn;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use serde_json::{Map, Value, json};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};
use std::ptr::write;

pub fn get_tag_value<T>(tag: Tag, obj: &DefaultDicomObject, def_value: T) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    obj.element_opt(tag)
        .ok()
        .flatten()
        .and_then(|e| e.to_str().ok())
        .and_then(|s| s.parse::<T>().ok())
        .unwrap_or(def_value)
}

pub fn get_string(tag: Tag, obj: &DefaultDicomObject) -> String {
    get_tag_value(tag, obj, "".to_string())
}

// pub fn tag_name(tag: Tag) -> String {
//     format!("{:?}", tag)
// }
pub fn json_key(tag: Tag) -> String {
    let group = tag.group();
    let elem = tag.element();
    format!("x{:04X}{:04X}", group, elem)
}

pub fn file_exists(p0: &PathBuf) -> bool {
    fs::metadata(p0).is_ok()
}
/// 递归遍历目录下的所有文件
pub fn walk_directory<P: Into<PathBuf>>(start_path: P) -> Result<Vec<PathBuf>, std::io::Error> {
    let start_path = start_path.into();
    let mut file_paths = Vec::new();

    if start_path.is_dir() {
        for entry in fs::read_dir(start_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // 递归处理子目录
                file_paths.extend(walk_directory(path)?);
            } else {
                // 收集文件路径
                file_paths.push(path);
            }
        }
    } else if start_path.is_file() {
        // 如果是单个文件，则直接添加
        file_paths.push(start_path);
    }

    Ok(file_paths)
}

pub fn change_transfer_syntax_iter(
    src: &PathBuf,
    dest: &PathBuf,
    ts: gdcm_conv::TransferSyntax,
    test_name: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    if !file_exists(src) {
        eprintln!("File does not exist: {:?}", src);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File or Directory does not exist: {:?}", src),
        )));
    }
    let test_root_name = test_name.unwrap_or("starSky14kWAKAME2k10X");
    let target_root = format!("{}/{}", dest.to_str().unwrap(), test_root_name);

    let target_path2 = format!(
        "{}/{}/{}/{}/{}.txt",
        dest.to_str().unwrap(),
        test_root_name,
        "2222",
        "3333",
        "12345.99"
    );
    // 获取父目录路径
    let target2_dir = Path::new(&target_path2).parent().unwrap();
    if fs::create_dir_all(&target2_dir).is_err() {
        eprintln!("Error creating directory: {:?}", dest);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            format!("Error creating directory: {:?}", dest),
        )));
    }
    if fs::write(&target_path2, "test").is_err() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            format!(
                "Error write file in Directory: {:?}-->{:?}",
                target_path2, target2_dir
            ),
        )));
    }
    // if fs::remove_file(&target_path2).is_err() {
    //     return Err(Box::new(std::io::Error::new(
    //         std::io::ErrorKind::PermissionDenied,
    //         format!(
    //             "Error remove file in Directory: {:?}-->{:?}",
    //             target_path2, target2_dir
    //         ),
    //     )));
    // }
    if fs::remove_dir_all(&target_root).is_err() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            format!("Error remove  Directory: {:?}", target_root),
        )));
    }

    let files = walk_directory(src)?;
    if files.is_empty() {
        eprintln!("No DICOM files found in the directory: {:?}", src);
        return Ok(());
    }

    files.par_iter().for_each(|file| {
        match open_file(file) {
            Ok(obj) => {
                let patient_id = get_string(tags::PATIENT_ID, &obj);
                let study_uid = get_string(tags::STUDY_INSTANCE_UID, &obj);
                let series_uid = get_string(tags::SERIES_INSTANCE_UID, &obj);
                let sop_uid = get_string(tags::SOP_INSTANCE_UID, &obj);
                let target_path = format!(
                    "{:?}/{:?}/{:?}/{:?}/{:?}.dcm",
                    dest, patient_id, study_uid, series_uid, sop_uid
                );
                // 获取父目录路径
                let target_dir = Path::new(&target_path).parent().unwrap();
                // let file_size= file.metadata().unwrap().len();

                // 递归创建目录（如果不存在）
                fs::create_dir_all(target_dir).unwrap();

                let mut input_buffer = Vec::with_capacity(512 * 512);
                // 将 DICOM 对象写入缓冲区,如果出错,则内存分配失败,直接退出
                obj.write_all(&mut input_buffer)
                    .expect("DefaultDicomObject write to buffer failed");

                // if let Err(e) = obj.write_all(&mut input_buffer) {
                //     eprintln!(
                //         "DefaultDicomObject write to buffer failed: {}",
                //         Report::from_error(e)
                //     );
                //     return;
                // }

                // obj.write_all(&mut buffer)
                //     .unwrap_or_else(|e| {
                //         eprintln!("Error writing to buffer: {}", Report::from_error(e));
                //     });
                match gdcm_conv::pipeline(
                    // Input DICOM file buffer
                    input_buffer,
                    // Estimated Length
                    None,
                    // First Transfer Syntax conversion
                    ts,
                    // Photometric conversion
                    PhotometricInterpretation::None,
                    // Second Transfer Syntax conversion
                    gdcm_conv::TransferSyntax::None,
                ) {
                    Ok(buffer) => {
                        let mut output_file = File::create(target_path).unwrap();
                        output_file.write_all(&buffer).unwrap();
                    }
                    Err(e) => {
                        eprintln!("Error during transcoding: {}", e);
                    }
                };
            }
            Err(e) => {
                eprintln!("Error opening file: {}", e);
                return;
            }
        };
    });
    Ok(())
}

#[warn(dead_code)]
pub fn change_transfer_syntax(
    src: &PathBuf,
    dest: &PathBuf,
    ts: gdcm_conv::TransferSyntax,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut dicoms: Vec<(FileDicomObject<InMemDicomObject>, PathBuf)> = Vec::new();

    match walk_directory(src) {
        Ok(files) => {
            for file in files {
                let obj = match open_file(&file) {
                    Ok(obj) => obj,
                    Err(e) => {
                        eprintln!("Error opening file: {}", e);
                        continue;
                    }
                };
                dicoms.push((obj, file));
            }
        }
        Err(e) => eprintln!("Error walking directory: {}", e),
    }
    for (obj, path) in dicoms.iter() {
        println!("Processing file: {:?}", path);
        let mut ibuffer = Vec::new();
        obj.write_all(&mut ibuffer).unwrap_or_else(|e| {
            eprintln!("Error writing to buffer: {}", Report::from_error(e));
        });
        match gdcm_conv::pipeline(
            // Input DICOM file buffer
            ibuffer,
            // Estimated Length
            None,
            // First Transfer Syntax conversion
            ts,
            // Photometric conversion
            PhotometricInterpretation::None,
            // Second Transfer Syntax conversion
            gdcm_conv::TransferSyntax::None,
        ) {
            Ok(buffer) => {
                let cursor = Cursor::new(buffer);
                // Read the DICOM object from the cursor
                let obj = dicom_object::from_reader(cursor).unwrap_or_else(|e| {
                    eprintln!("Error reading DICOM object: {}", e);
                    std::process::exit(1);
                });
                println!(
                    "Patient Name: {:?}",
                    get_string(tags::SOP_INSTANCE_UID, &obj)
                );
                let target_path = format!(
                    "{}/{}.dcm",
                    dest.to_str().unwrap(),
                    path.file_stem().unwrap().to_str().unwrap()
                );
                obj.write_to_file(target_path).unwrap_or_else(|e| {
                    eprintln!("{}", Report::from_error(e));
                });
            }
            Err(e) => {
                eprintln!("Error during transcoding: {}", e);
                return Err(Box::new(e));
            }
        };
    }
    Ok(())
}

pub fn convert_ts_with_gdcm(
    p0: &PathBuf,
    output_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    // 步骤 1: 读取 DICOM 文件
    use gdcm_conv::{PhotometricInterpretation, TransferSyntax};
    use std::fs::File;
    use std::io::prelude::*;

    // Read input file
    let mut ibuffer = Vec::new();
    let mut ifile = File::open(p0).unwrap();
    ifile.read_to_end(&mut ibuffer).unwrap();

    // Transcode DICOM file
    let buffer = match gdcm_conv::pipeline(
        // Input DICOM file buffer
        ibuffer,
        // Estimated Length
        None,
        // First Transfer Syntax conversion
        TransferSyntax::JPEG2000Lossless,
        // Photometric conversion
        PhotometricInterpretation::None,
        // Second Transfer Syntax conversion
        TransferSyntax::None,
    ) {
        Ok(buffer) => buffer,
        Err(e) => {
            eprintln!("Error during transcoding: {}", e);
            return Err(Box::new(e));
        }
    };

    // 转换完毕后执行验证
    // Wrap Vec<u8> in a Cursor to implement Read
    let cursor = Cursor::new(buffer);

    // Read the DICOM object from the cursor
    let obj = dicom_object::from_reader(cursor)?;

    // Now you can use `obj` as a DICOM object
    println!("Patient Name: {:?}", get_string(tags::PATIENT_NAME, &obj));
    println!("Patient Name: {:?}", get_string(tags::PATIENT_ID, &obj));
    println!(
        "Patient Name: {:?}",
        get_string(tags::STUDY_DESCRIPTION, &obj)
    );

    obj.write_to_file(output_path).unwrap_or_else(|e| {
        eprintln!("{}", Report::from_error(e));
    });
    //  直接写入文件的方式
    // let mut output_file = File::create(output_path).unwrap();
    // output_file.write_all(&buffer).unwrap();

    Ok(())
}

// 此修改文件传输语法,存在限制.
#[warn(dead_code)]
pub fn convert_ts_with_pixel_data(
    file: &PathBuf,
    output_path: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // 步骤 1: 读取 DICOM 文件
    let mut obj = open_file(file).unwrap_or_else(|e| {
        eprintln!("open file error :{}", Report::from_error(e));
        std::process::exit(1);
    });
    assert_eq!(
        obj.meta().transfer_syntax(),
        uids::IMPLICIT_VR_LITTLE_ENDIAN
    );

    // Transcode DICOM file
    // let ts = dicom_transfer_syntax_registry::TransferSyntaxRegistry
    //     .get(uids::JPEG_BASELINE8_BIT)
    //     .unwrap();

    // let mut options = EncodeOptions::default();
    // options.quality = Some(100);
    // options.effort = Some(100);
    // obj.transcode_with_options(ts, options).unwrap_or_else(|e| {
    //     eprintln!("{}", Report::from_error(e));
    // });
    // transcode to JPEG baseline
    let target_ts = dicom_transfer_syntax_registry::entries::JPIP_HTJ2K_REFERENCED_DEFLATE;
    println!(
        "Target TS :{:?} , is_codec_free : {:?}",
        target_ts.name(),
        target_ts.is_codec_free()
    );
    if target_ts.is_codec_free() {
        // transcode without codec
        obj.transcode(&target_ts.erased())
            .expect("Should have transcode successfully");
        obj.update_meta(|meta| {
            meta.implementation_class_uid = dicom_object::IMPLEMENTATION_CLASS_UID.to_string();
            meta.implementation_version_name =
                Some(dicom_object::IMPLEMENTATION_VERSION_NAME.to_string());
        });

        obj.write_to_file(output_path).unwrap_or_else(|e| {
            eprintln!("{}", Report::from_error(e));
        });
    } else {
        // 添加错误提示
        eprintln!("Error: 不支持的传输语法: {}", target_ts.uid());
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            format!("不支持的传输语法:{:?}", target_ts.uid()),
        )));
    }
    Ok(())
}

pub fn generate_json_file(file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if !file_exists(file) {
        eprintln!("File does not exist: {:?}", file);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File or Directory does not exist: {:?}", file),
        )));
    }
    if !file.is_dir() {
        // 递归遍历目录下的所有文件
        eprintln!("File does not exist: {:?}", file);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File or Directory does not exist: {:?}", file),
        )));
    }
    let files = walk_directory(file)?;
    if files.is_empty() {
        eprintln!("No DICOM files found in the directory: {:?}", file);
        return Ok(());
    }
    let mut seris_map: HashMap<(String, u32),  Value> = HashMap::new();
    let mut sop_map: HashMap<(String, String,u32),  Value> = HashMap::new();
    for file in files {
        let obj = OpenFileOptions::new()
            .read_until(tags::PIXEL_DATA)
            .open_file(&file)
            .unwrap_or_else(|e| {
                eprintln!("open file error :{}", Report::from_error(e));
                std::process::exit(1);
            });
        let series_uid = get_string(tags::SERIES_INSTANCE_UID, &obj);
        let exists = seris_map.keys().any(|(uid, _)| uid == &series_uid);
        
        if !exists {
            let sex = get_string(tags::PATIENT_SEX, &obj);
            let age = get_string(tags::PATIENT_AGE, &obj);
            let name = get_string(tags::PATIENT_NAME, &obj);
            let paid = get_string(tags::PATIENT_ID, &obj);
            let birth_date = get_string(tags::PATIENT_BIRTH_DATE, &obj);
            let modality = get_string(tags::MODALITY, &obj);
            let body_part = get_string(tags::BODY_PART_EXAMINED, &obj);
            let sn = get_string(tags::SERIES_NUMBER, &obj);
            let study_date = get_string(tags::STUDY_DATE, &obj);
            let study_time = get_string(tags::STUDY_TIME, &obj);
            let acc_num = get_string(tags::ACCESSION_NUMBER, &obj);
            let manufacturer = get_string(tags::MANUFACTURER, &obj);
            let institution_address = get_string(tags::INSTITUTION_ADDRESS, &obj);
            let institution_name = get_string(tags::INSTITUTION_NAME, &obj);
            let series_num =  sn.parse::<u32>().unwrap_or(0);
            let series_json = json!({ 
                  "00100040": sex,
                  "00101010": age,
                  "0020000E": series_uid,
                  "00100010": name,
                  "00100020": paid,
                  "00100030": birth_date,
                  "00180015": body_part,
                  "00200011": sn ,
                  "00080020": study_date,
                  "00080030": study_time,
                  "00080050": acc_num,
                  "00080060": modality,
                  "00080070": manufacturer,
                  "00080081": institution_address,
                  "00080080": institution_name,
            });
            seris_map.insert((series_uid.clone(), series_num), series_json);
        }

        let series_desc = get_string(tags::SERIES_DESCRIPTION, &obj);
        let pixel_spacing = get_string(tags::PIXEL_SPACING, &obj);
        let px_spacing_vec: Vec<&str> = pixel_spacing.split("\\").collect();
        let rows = get_string(tags::ROWS, &obj);
        let columns = get_string(tags::COLUMNS, &obj);
        let body_part = get_string(tags::BODY_PART_EXAMINED, &obj);
        let image_type = get_string(tags::IMAGE_TYPE, &obj);
        let image_type_vec: Vec<&str> = image_type.split("\\").collect();
        let pixel_representation = get_string(tags::PIXEL_REPRESENTATION, &obj);
        let patient_position = get_string(tags::PATIENT_POSITION, &obj);
        let image_position_patient = get_string(tags::IMAGE_POSITION_PATIENT, &obj);
        let image_position_patient_vec: Vec<&str> = image_position_patient.split("\\").collect();
        let image_orientation_patient = get_string(tags::IMAGE_ORIENTATION_PATIENT, &obj);
        let image_orientation_patient_vec: Vec<&str> =
            image_orientation_patient.split("\\").collect();

        let instance_num = get_string(tags::INSTANCE_NUMBER, &obj);
        let slice_thickness = get_string(tags::SLICE_THICKNESS, &obj);
        let sop_uid = get_string(tags::SOP_INSTANCE_UID, &obj);
        let inst_num =  instance_num.parse::<u32>().unwrap_or(0);
        let sop_json = json!({
          "0008103E": series_desc ,
          "00280030": px_spacing_vec,
          "00280010": rows  ,
          "00280011": columns,
          "00180015": body_part,
          "00080008": image_type_vec,
          "00280103": pixel_representation,
          "00185100": patient_position,
          "00200032": image_position_patient_vec,
          "00180050": slice_thickness,
          "00200013": instance_num,
          "00200037": image_orientation_patient_vec,
          "00080018": sop_uid
        });
        sop_map.insert((series_uid, sop_uid,inst_num), sop_json );
    }

    let mut study_vec = Vec::new();
    // 排序后的 series (series_uid, series_num, series_json)
    let mut seris_vec: Vec<(&(String, u32), &Value)> = seris_map.iter().collect();
    seris_vec.sort_by_key(|((_, series_num), _)| *series_num); 
    
    for ((series_uid, _series_num), series_json) in seris_vec  {
        // 1. 收集、排序
        let mut sop_list: Vec<(u32, &Value)> = sop_map.iter()
            .filter(|((s_uid, _, _), _)| s_uid == series_uid)
            .map(|((_, _, inst_num), sop_json)| (*inst_num, sop_json))
            .collect();
        sop_list.sort_by_key(|(inst_num, _)| *inst_num);
        let sop_vec: Vec<&Value> = sop_list.into_iter().map(|(_, v)| v).collect();
        
        // let mut sop_vec = Vec::new();
        // for (s_uid, sop_json) in sop_map.iter() {
        //     if s_uid.0 == *series_uid {
        //         sop_vec.push(sop_json);
        //     }
        // }
        // 2. 组装 series
        let series_json = series_json.as_object().unwrap();
        let json_str =  json!({
              "00100040": series_json["00100040"],
              "00101010": series_json["00101010"],
              "0020000E": series_uid,
              "00100010": series_json["00100010"],
              "00100020": series_json["00100020"],
              "00100030": series_json["00100030"],
              "00180015": series_json["00180015"],
              "00200011": series_json["00200011"] ,
              "00080020": series_json["00080020"],
              "00080030": series_json["00080030"],
              "00080050": series_json["00080050"],
              "00080060": series_json["00080060"],
              "00080070": series_json["00080070"],
              "00080081": series_json["00080081"],
              "00080080": series_json["00080080"],
              "sopData":sop_vec
        }); 
        study_vec.push(json_str);
         
    }
    
    let  study_json= json!({
        "seriesData": study_vec,
        "hiscode":"89269",
        "expires":"2025-06-20T13-05-16",
        "token":"cbcbc2c203fe3877737c0befd6a769fa" 
     });
    write_json_file2(&study_json.to_string(), "./tst22.json");
    return Ok(());
}

// fn write_json_file(p0: &mut Map<String, Value>, p1: &str) {
//     let json_value = Value::Object(p0.clone());
//     let json_string = serde_json::to_string_pretty(&json_value);
//     let file = File::create(p1);
//     if (file.is_ok() && json_string.is_ok()) {
//         file.unwrap()
//             .write_all(json_string.unwrap().as_bytes())
//             .unwrap();
//     }
// }
fn write_json_file2(json: &String, p1: &str) {
    
   
    let file = File::create(p1);
    if (file.is_ok()  ) {
        file.unwrap()
            .write_all( json.as_bytes())
            .unwrap();
    }
}