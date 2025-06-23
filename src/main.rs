mod dcm_meta;
mod dcmobj;
mod dicom_info;
mod image_info;
mod patient_info;
mod series_info;
mod study_info;
 

use crate::dcmobj::{change_transfer_syntax_iter, file_exists, generate_json_file};
use clap::Parser;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug, Parser)]
#[command(version)]
struct Application {
    /// Verbose mode
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
    #[arg(short = 'i', long = "dicom-file", required = true)]
    input_file: PathBuf,
    #[arg(short = 'o', long = "json-file", required = true)]
    output_file: PathBuf,
}
fn main() {
    println!("Hello, world!");
    let app = Application::parse();
    println!("{:?}", app);

    if !file_exists(&app.input_file) {
        println!("{}: File  not exists", app.input_file.to_str().unwrap());
        return;
    }
    println!("{}: File  exists", app.input_file.to_str().unwrap());
    println!("{}: target file ", app.output_file.to_str().unwrap());
    let start = Instant::now();
    change_transfer_syntax_iter(
        &app.input_file,
        &app.output_file,
        gdcm_conv::TransferSyntax::JPEG2000Lossless,
        None,
    )
    .unwrap();
    let duration = start.elapsed();
    println!("耗时: {} 秒", duration.as_secs_f64());
    println!("耗时: {} 毫秒", duration.as_millis());


    let di_com = PathBuf::from("/home/dhz/Documents/JSONDS");
    generate_json_file(&di_com).unwrap();
}

// fn convert_ts(p0: &PathBuf, output_path: String) -> Result<(), Box<dyn std::error::Error>> {
//     // 步骤 1: 读取 DICOM 文件
//     let obj = OpenFileOptions::new()
//         .read_until(tags::PIXEL_DATA)
//         .open_file(p0)
//         .unwrap();
//     // clone 一份对象用于构建新文件
//     let meta = obj.meta().clone();
//
//     let sop_class_uid = meta.media_storage_sop_class_uid;
//     let sop_inst_uid = meta.media_storage_sop_instance_uid;
//
//     // 步骤 3: 创建一个新的 DICOM 文件对象并设置传输语法
//     let file_obj = FileMetaTableBuilder::new()
//         .media_storage_sop_class_uid(sop_class_uid)
//         .media_storage_sop_instance_uid(sop_inst_uid)
//         .transfer_syntax(dicom_transfer_syntax_registry::entries::EXPLICIT_VR_LITTLE_ENDIAN.uid())
//         .implementation_class_uid(IMPLEMENTATION_CLASS_UID)
//         .implementation_version_name(IMPLEMENTATION_VERSION_NAME)
//         .build()
//         .unwrap();
//
//     // 注意 with_meta 返回新对象（有的版本直接返回 Result）
//     let file_obj = DefaultDicomObject::new_empty_with_meta(file_obj);
//     file_obj.write_to_file(output_path)?;
//     println!("传输语法已修改并保存到  ");
//     Ok(())
// }
//

#[cfg(test)]
mod tests {
    use crate::Application;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Application::command().debug_assert();
    }

    use super::*;
    use crate::dcm_meta::{DcmMapMeta, DcmMeta};
    use crate::dcmobj::convert_ts_with_gdcm;
    use crate::dicom_info::DicomInfo;
    use crate::image_info::ImageInfo;
    use crate::patient_info::PatientInfo;
    use crate::series_info::SeriesInfo;
    use crate::study_info::StudyInfo;
    use dicom::dictionary_std::tags;
    use dicom_object::OpenFileOptions;
    use std::fs::File;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_file_exists_when_file_exists() {
        // 创建一个临时目录
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_file.txt");

        // 创建测试文件
        File::create(&file_path).unwrap();

        // 验证函数返回 true
        assert!(file_exists(&file_path));

        // 临时目录会在离开作用域时自动删除
    }

    #[test]
    fn test_file_exists_when_file_not_exists() {
        let non_existent_path = PathBuf::from("/path/that/does/not/exist");

        // 验证函数返回 false
        assert!(!file_exists(&non_existent_path));
    }

    #[test]
    fn test_file_exists_when_path_is_directory() {
        // 创建一个临时目录
        let dir = tempdir().unwrap();

        // 验证函数对目录返回 true (因为目录也是一种文件系统对象)
        assert!(file_exists(&dir.path().to_path_buf()));
    }

    #[test]
    fn test_file_read() {
        let input_path =
            PathBuf::from("./test_data/1.2.840.113713.55902.1.642704.2248.1270609320.4549.dcm");
        let output_path =
            PathBuf::from("./test_data/A_1.2.840.113713.55902.1.642704.2248.1270609320.4549.dcm");
        let json_path =
            PathBuf::from("./test_data/A_1.2.840.113713.55902.1.642704.2248.1270609320.4549.json");
        let obj = OpenFileOptions::new()
            .read_until(tags::PIXEL_DATA)
            .open_file(&input_path)
            .unwrap();
        // let obj = open_file(&app.input_file).unwrap();
        let patient_id_exists = obj.element_opt(tags::PATIENT_ID).is_ok();
        let study_uid_exists = obj.element_opt(tags::STUDY_INSTANCE_UID).is_ok();
        let series_uid_exists = obj.element_opt(tags::SERIES_INSTANCE_UID).is_ok();
        let sop_uid_exists = obj.element_opt(tags::SOP_INSTANCE_UID).is_ok();
        if !patient_id_exists || !study_uid_exists || !series_uid_exists || !sop_uid_exists {
            println!(" Tags: PatientId, StudyUID, SeriesUID,SopInstanceUID is not exists !");
            return;
        }
        let uuid = obj.meta().transfer_syntax();
        println!("Transfer Syntax UID: [{}]", uuid);

        let patient_info = PatientInfo::new(&obj);
        println!("PatientInfo: {:?}", patient_info);
        let study_info = StudyInfo::new(&obj);
        println!("StudyInfo: {:?}", study_info);
        let series_info = SeriesInfo::new(&obj);
        println!("SeriesInfo: {:?}", series_info);
        let image_info = ImageInfo::new(&obj);
        println!("ImageInfo: {:?}", image_info);

        let dicom_info = DicomInfo::new(&obj);
        println!("DicomInfo: {:?}", dicom_info);
        let metadata = DcmMeta::new(&obj);
        println!("DicomInfo: {:?}", metadata);
        let map_tag = DcmMapMeta::new(&obj);
        println!("DicomInfo: {:?}-->{:?}", map_tag, map_tag.check_valid());
        let json = serde_json::to_string(&dicom_info).unwrap();
        std::fs::write(json_path, json).unwrap();

        convert_ts_with_gdcm(&input_path, &output_path).unwrap();
    }
}
