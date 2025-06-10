use std::io::Cursor;
use std::path::PathBuf;
use dicom::core::Tag;
use dicom::dictionary_std::{tags, uids};
use dicom::object::DefaultDicomObject;
use dicom::pixeldata::Transcode;
use dicom_encoding::snafu::Report;
use dicom_object::open_file;

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
    std::fs::metadata(p0).is_ok()
}


pub fn convert_ts_with_gdcm(
    p0: &PathBuf,
    output_path: String,
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
    } else{
        // 添加错误提示
        eprintln!("Error: 不支持的传输语法: {}", target_ts.uid());
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Unsupported,

            format!("不支持的传输语法:{:?}",target_ts.uid())
        )));
    }
    Ok(())
}