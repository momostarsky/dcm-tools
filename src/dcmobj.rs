use dicom::core::Tag;
use dicom::dictionary_std::{tags, uids};
use dicom::object::DefaultDicomObject;
use dicom::pixeldata::Transcode;
use dicom_encoding::snafu::Report;
use dicom_object::{open_file, FileDicomObject, InMemDicomObject};
use gdcm_conv::PhotometricInterpretation;
use log::warn;
use std::fs;
use std::io::{Cursor};
use std::path::PathBuf;

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
pub fn change_transfer_syntax(
    src: &PathBuf,
    dest: &PathBuf,
    ts: gdcm_conv::TransferSyntax,
) -> Result<(), Box<dyn std::error::Error>> {
    if !file_exists(src) {
        eprintln!("File does not exist: {:?}", src);
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File does not exist: {:?}", src),
        )))?;
    }
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
