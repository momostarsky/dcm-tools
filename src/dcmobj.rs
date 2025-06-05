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
    pub series_date: String,
    pub series_time: String,
    pub series_desc: String,
    pub modality: String,
    pub body_part_exam: String,
    pub image_type: String,
    pub accession_number: String,
}
impl SeriesInfo {
    // 为 PatientInfo 添加构造函数
    pub fn new(obj: &DefaultDicomObject) -> Self {
        Self {
            series_num: get_tag_value(tags::SERIES_NUMBER, obj, "".to_string()),
            series_uid: get_tag_value(tags::SERIES_INSTANCE_UID, obj, "".to_string()),
            series_date: get_tag_value(tags::SERIES_DATE, obj, "".to_string()),
            series_time: get_tag_value(tags::SERIES_TIME, obj, "".to_string()),
            series_desc: get_tag_value(tags::SERIES_DESCRIPTION, obj, "".to_string()),
            modality: get_tag_value(tags::MODALITY, obj, "".to_string()),
            body_part_exam: get_tag_value(tags::BODY_PART_EXAMINED, obj, "".to_string()),
            image_type: get_tag_value(tags::IMAGE_TYPE, obj, "".to_string()),
            accession_number: get_tag_value(tags::ACCESSION_NUMBER, obj, "".to_string()),
        }
    }
}
#[derive(Debug, Parser, Clone)]
pub struct ImageInfo {
    pub inst_num: u16,
    pub sop_uid: String,
    pub rows: u16,
    pub cols: u16,
    pub pixel_spacing: String,
    pub slice_location: String,
    pub slice_thickness: String,
    pub image_position: String,
    pub image_orientation: String,
    pub window_center: String,
    pub window_width: String,
    pub photometric_interpretation: String,
    pub bits_allocated: u16,
    pub high_bit: u16,
    pub bit_stored: u16,
    pub samples_per_pixel: u16,
    pub pixel_representation: u16,
}

impl ImageInfo {
    // 为 PatientInfo 添加构造函数
    pub fn new(obj: &DefaultDicomObject) -> Self {
        Self {
            inst_num: get_tag_value(tags::INSTANCE_NUMBER, obj, 1),
            sop_uid: get_tag_value(tags::SOP_INSTANCE_UID, obj, "".to_string()),
            rows: get_tag_value(tags::ROWS, obj, 0),
            cols: get_tag_value(tags::COLUMNS, obj, 0),
            pixel_spacing: get_tag_value(tags::PIXEL_SPACING, obj, "".to_string()),
            slice_location: get_tag_value(tags::SLICE_LOCATION, obj, "".to_string()),
            slice_thickness: get_tag_value(tags::SLICE_THICKNESS, obj, "".to_string()),
            image_position: get_tag_value(tags::IMAGE_POSITION_PATIENT, obj, "".to_string()),
            image_orientation: get_tag_value(tags::IMAGE_ORIENTATION_PATIENT, obj, "".to_string()),
            window_center: get_tag_value(tags::WINDOW_CENTER, obj, "".to_string()),
            window_width: get_tag_value(tags::WINDOW_WIDTH, obj, "".to_string()),
            photometric_interpretation: get_tag_value(
                tags::PHOTOMETRIC_INTERPRETATION,
                obj,
                "".to_string(),
            ),
            bits_allocated: get_tag_value(tags::BITS_ALLOCATED, obj, 8),
            high_bit: get_tag_value(tags::HIGH_BIT, obj, 7),
            bit_stored: get_tag_value(tags::BITS_STORED, obj, 7),
            samples_per_pixel: get_tag_value(tags::SAMPLES_PER_PIXEL, obj, 1),
            pixel_representation: get_tag_value(tags::PIXEL_REPRESENTATION, obj, 1),
        }
    }
}
