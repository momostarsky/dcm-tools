use clap::Parser;
use dicom::core::Tag;
use dicom::dictionary_std::tags;
use dicom::object::DefaultDicomObject;

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

#[derive(Debug, Parser, Clone)]
pub struct PatientInfo {
    pub id: String,
    pub name: String,
    pub age: String,
    pub sex: String,
    pub birth_date: String,
    pub birth_time: String,
}

impl PatientInfo {
    // 为 PatientInfo 添加构造函数
    pub fn new(obj: &DefaultDicomObject) -> Self {
        Self {
            id: get_tag_value(tags::PATIENT_ID, obj, "".to_string()),
            name: get_tag_value(tags::PATIENT_NAME, obj, "".to_string()),
            age: get_tag_value(tags::PATIENT_AGE, obj, "".to_string()),
            sex: get_tag_value(tags::PATIENT_SEX, obj, "".to_string()),
            birth_date: get_tag_value(tags::PATIENT_BIRTH_DATE, obj, "".to_string()),
            birth_time: get_tag_value(tags::PATIENT_BIRTH_TIME, obj, "".to_string()),
        }
    }
}

#[derive(Debug, Parser, Clone)]
pub struct StudyInfo {
    pub study_id: String,
    pub study_uid: String,
    pub study_date: String,
    pub study_time: String,
    pub study_desc: String,
}
impl StudyInfo {
    // 为 PatientInfo 添加构造函数
    pub fn new(obj: &DefaultDicomObject) -> Self {
        Self {
            study_id: get_tag_value(tags::STUDY_ID, obj, "".to_string()),
            study_uid: get_tag_value(tags::STUDY_INSTANCE_UID, obj, "".to_string()),
            study_date: get_tag_value(tags::STUDY_DATE, obj, "".to_string()),
            study_time: get_tag_value(tags::STUDY_TIME, obj, "".to_string()),
            study_desc: get_tag_value(tags::STUDY_DESCRIPTION, obj, "".to_string()),
        }
    }
}
#[derive(Debug, Parser, Clone)]
pub struct SeriesInfo {
   pub series_num: String,
   pub series_uid: String,
   pub series_desc: String,
   pub modality: String,
   pub exam_body_part: String,
   pub image_type: String,
   pub accession_number: String,
}
impl SeriesInfo {
    // 为 PatientInfo 添加构造函数
    pub fn new(obj: &DefaultDicomObject) -> Self {
        Self {
            series_num: get_tag_value(tags::SERIES_NUMBER, obj, "".to_string()),
            series_uid: get_tag_value(tags::SERIES_INSTANCE_UID, obj, "".to_string()),
            series_desc: get_tag_value( tags::SERIES_DESCRIPTION, obj, "".to_string()),
            modality: "".to_string(),
            exam_body_part: "".to_string(),
            image_type: "".to_string(),
            accession_number: "".to_string(),
        }
    }
}
#[derive(Debug, Parser, Clone)]
pub struct ImageInfo {
   pub inst_num: String,
   pub sop_uid: String,
   pub rows: String,
   pub cols: String,
   pub pixel_spacing: String,
   pub slice_location: String,
   pub slice_thickness: String,
   pub image_position: String,
   pub image_orientation: String,
   pub window_center: String,
   pub window_width: String,
   pub photometric_interpretation: String,
   pub bits_allocated: String,
   pub high_bit: String,
   pub bit_stored: String,
   pub samples_per_pixel: String,
   pub pixel_representation: String,
}

impl ImageInfo {
    // 为 PatientInfo 添加构造函数
    pub fn new(obj: &DefaultDicomObject) -> Self {
        Self {
            inst_num: get_tag_value(tags::INSTANCE_NUMBER, obj, "".to_string()),
            sop_uid: get_tag_value(tags::SOP_INSTANCE_UID, obj, "".to_string()),
            rows: get_tag_value(tags::ROWS, obj, "".to_string()),
            cols: get_tag_value(tags::COLUMNS, obj, "".to_string()),
            pixel_spacing: get_tag_value(tags::PIXEL_SPACING, obj, "".to_string()),
            slice_location: get_tag_value(tags::SLICE_LOCATION, obj, "".to_string()),
            slice_thickness: get_tag_value(tags::SLICE_THICKNESS, obj, "".to_string()),
            image_position: get_tag_value(tags::IMAGE_POSITION_PATIENT, obj, "".to_string()),
            image_orientation: get_tag_value(tags::IMAGE_ORIENTATION_PATIENT, obj, "".to_string()),
            window_center: get_tag_value(tags::WINDOW_CENTER, obj, "".to_string()),
            window_width: get_tag_value(tags::WINDOW_WIDTH, obj, "".to_string()),
            photometric_interpretation: get_tag_value(tags::PHOTOMETRIC_INTERPRETATION, obj, "".to_string()),
            bits_allocated: get_tag_value(tags::BITS_ALLOCATED, obj, "".to_string()),
            high_bit: get_tag_value(tags::HIGH_BIT, obj, "".to_string()),
            bit_stored: get_tag_value(tags::BITS_STORED, obj, "".to_string()),
            samples_per_pixel: get_tag_value(tags::SAMPLES_PER_PIXEL, obj, "".to_string()),
            pixel_representation: get_tag_value(tags::PIXEL_REPRESENTATION, obj, "".to_string()),   
        }
    }
}