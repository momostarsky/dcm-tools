mod dcmobj;
mod image_info;
mod series_info;
mod study_info;
mod patient_info;
mod dicom_info;

use crate::image_info::ImageInfo;
use crate::series_info::SeriesInfo;
use crate::study_info::StudyInfo;
use clap::Parser;
use dicom::dictionary_std::tags;
use dicom::object::OpenFileOptions;
use std::path::PathBuf;
use crate::dicom_info::DicomInfo;
use crate::patient_info::PatientInfo;

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
    let input_path = &app.input_file;
    let obj = OpenFileOptions::new()
        .read_until(tags::PIXEL_DATA)
        .open_file(input_path)
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

    let json = serde_json::to_string(&dicom_info).unwrap();
    std::fs::write(&app.output_file, json).unwrap();
    
}

fn file_exists(p0: &PathBuf) -> bool {
    std::fs::metadata(p0).is_ok()
}

#[cfg(test)]
mod tests {
    use crate::Application;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Application::command().debug_assert();
    }

    use super::*;
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
}
