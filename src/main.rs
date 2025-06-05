mod dcmobj;

use clap::Parser;
use dicom::dictionary_std::tags;
// use dicom::object::open_file;
// use clap::builder::TypedValueParser;
use dicom::core::Tag;
use dicom::object::OpenFileOptions;
use std::path::PathBuf;
use crate::dcmobj::{get_tag_value, PatientInfo};

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
    let patient_info = PatientInfo::new(&obj);
    println!("PatientInfo: {:?}", patient_info);
    
    let modality = obj.element(tags::MODALITY).unwrap();
    let study_uid = obj.element(tags::STUDY_INSTANCE_UID).unwrap();
    let series_uid = obj.element(tags::SERIES_INSTANCE_UID).unwrap();
    let sop_uid = obj.element(tags::SOP_INSTANCE_UID).unwrap();
    let study_date = obj.element(tags::STUDY_DATE).unwrap();
    let rows = obj.element(tags::ROWS).unwrap();
    let cols = obj.element(tags::COLUMNS).unwrap();
    let wc = obj.element(tags::WINDOW_CENTER).unwrap();
    let wl = obj.element(tags::WINDOW_WIDTH).unwrap();
    let photometric_interpretation = obj.element(tags::PHOTOMETRIC_INTERPRETATION).unwrap();
    let bits_allocated: i16 = get_tag_value(tags::BITS_ALLOCATED, &obj, 8);
    let high_bit: i16 = get_tag_value(tags::BITS_ALLOCATED, &obj, 7);
    let bit_stored: i16 = get_tag_value(tags::BITS_ALLOCATED, &obj, 7);
    let samples_per_pixel: i16 = get_tag_value(tags::SAMPLES_PER_PIXEL, &obj, 1);
    let pixel_representation: i16 = get_tag_value(tags::PIXEL_REPRESENTATION, &obj, 7);

    let acc_num = obj.element(tags::ACQUISITION_NUMBER).unwrap();
    let study_id = obj.element(tags::STUDY_ID).unwrap();
    let study_desc = obj.element(tags::STUDY_DESCRIPTION).unwrap();
    let series_num = obj.element(tags::SERIES_NUMBER).unwrap();
    let series_desc = obj.element(tags::SERIES_DESCRIPTION).unwrap();
    let image_type = obj.element(tags::IMAGE_TYPE).unwrap();
    let inst_num = get_tag_value(tags::INSTANCE_NUMBER, &obj, 1);

    let pixel_spacing = obj.element(tags::PIXEL_SPACING).unwrap();
    let slice_location = obj.element(tags::SLICE_LOCATION).unwrap();
    let slice_thickness = obj.element(tags::SLICE_THICKNESS).unwrap();
    let image_position = obj.element(tags::IMAGE_POSITION_PATIENT).unwrap();
    let image_orientation = obj.element(tags::IMAGE_ORIENTATION_PATIENT).unwrap();

   
    println!("modality:{:?}", modality.to_str());
    println!("study_uid:{:?}", study_uid.to_str());
    println!("series_uid:{:?}", series_uid.to_str());
    println!("sop_uid:{:?}", sop_uid.to_str());
    println!("study_date:{:?}", study_date.to_str());
    let output_path = &app.output_file;
    let json_str = format!(
        r#"{{ 
            "modality": "{}",
            "study_uid": "{}",
            "series_uid": "{}",
            "sop_uid": "{}",
            "study_date": "{}",
            "rows": "{}",
            "cols": "{}",
            "windows_center": "{}",
            "windows_width": "{}",
            "instance_number:  {},
            "study_id": "{}",
            "study_desc": "{}",
            "series_num": "{}",
            "series_desc": "{}",
            "acc_num": "{}",
            "image_type": "{}",
            "photometric_interpretation": "{}",
            "bits_allocated": {},
            "high_bit": {},
            "bit_stored": {},
            "samples_per_pixel": {},
            "pixel_representation": {},
            "pixel_spacing": "{}",
            "slice_location": "{}",
            "slice_thickness": "{}",
            "image_position": "{}",
            "image_orientation": "{}"   
        }}"#,
       
        modality.to_str().unwrap(),
        study_uid.to_str().unwrap(),
        series_uid.to_str().unwrap(),
        sop_uid.to_str().unwrap(),
        study_date.to_str().unwrap(),
        rows.to_str().unwrap(),
        cols.to_str().unwrap(),
        wc.to_str().unwrap(),
        wl.to_str().unwrap(),
        inst_num,
        study_id.to_str().unwrap(),
        study_desc.to_str().unwrap(),
        series_num.to_str().unwrap(),
        series_desc.to_str().unwrap(),
        acc_num.to_str().unwrap(),
        image_type.to_str().unwrap(),
        photometric_interpretation.to_str().unwrap(),
        bits_allocated,
        high_bit,
        bit_stored,
        samples_per_pixel,
        pixel_representation,
        pixel_spacing.to_str().unwrap(),
        slice_location.to_str().unwrap(),
        slice_thickness.to_str().unwrap(),
        image_position.to_str().unwrap(),
        image_orientation.to_str().unwrap(),
    );
    assert!(std::str::from_utf8(json_str.as_bytes()).is_ok());
    std::fs::write(output_path, json_str).expect("Unable to write file");
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
