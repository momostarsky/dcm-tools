use clap::Parser;
use dicom::core::Tag;
use dicom::dictionary_std::tags;
use dicom::object::DefaultDicomObject;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

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
    return get_tag_value(tag, obj, "".to_string());
}
pub fn tag_name(tag: Tag) -> String {
    format!("{:?}", tag)
}
pub fn json_key(tag: Tag) -> String {
    let group = tag.group();
    let elem = tag.element();
    format!("x{:04X}{:04X}", group, elem)
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
            id: get_string(tags::PATIENT_ID, obj),
            name: get_string(tags::PATIENT_NAME, obj),
            age: get_string(tags::PATIENT_AGE, obj),
            sex: get_string(tags::PATIENT_SEX, obj),
            birth_date: get_string(tags::PATIENT_BIRTH_DATE, obj),
            birth_time: get_string(tags::PATIENT_BIRTH_TIME, obj),
        }
    }
    fn map_key(tag: Tag) -> &'static str {
        match tag {
            tags::PATIENT_ID => "x00100020",
            tags::PATIENT_NAME => "x00100010",
            tags::PATIENT_AGE => "x00101010",
            tags::PATIENT_SEX => "x00100040",
            tags::PATIENT_BIRTH_DATE => "x00100030",
            tags::PATIENT_BIRTH_TIME => "x00100032",
            _ => "unknown",
        }
    }
}
// Assume implementing the Serialize trait here
impl Serialize for PatientInfo {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = s.serialize_struct("PatientInfo", 6)?;
        // 使用内联方式避免局部变量生命周期问题
        state.serialize_field(Self::map_key(tags::PATIENT_ID), &self.id)?;
        state.serialize_field(Self::map_key(tags::PATIENT_NAME), &self.name)?;
        state.serialize_field(Self::map_key(tags::PATIENT_AGE), &self.age)?;
        state.serialize_field(Self::map_key(tags::PATIENT_SEX), &self.sex)?;
        state.serialize_field(Self::map_key(tags::PATIENT_BIRTH_DATE), &self.birth_date)?;
        state.serialize_field(Self::map_key(tags::PATIENT_BIRTH_TIME), &self.birth_time)?;
        state.end()
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
            study_id: get_string(tags::STUDY_ID, obj),
            study_uid: get_string(tags::STUDY_INSTANCE_UID, obj),
            study_date: get_string(tags::STUDY_DATE, obj),
            study_time: get_string(tags::STUDY_TIME, obj),
            study_desc: get_string(tags::STUDY_DESCRIPTION, obj),
        }
    }

    fn map_key(tag: Tag) -> &'static str {
        match tag {
            tags::STUDY_ID => "x00200010",
            tags::STUDY_INSTANCE_UID => "x0020000D",
            tags::STUDY_DATE => "x00080020",
            tags::STUDY_TIME => "x00080030",
            tags::STUDY_DESCRIPTION => "x00081030",
            _ => "unknown",
        }
    }
}
impl Serialize for StudyInfo {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = s.serialize_struct("StudyInfo", 5)?;
        state.serialize_field(Self::map_key(tags::STUDY_ID), &self.study_id)?;
        state.serialize_field(Self::map_key(tags::STUDY_INSTANCE_UID), &self.study_uid)?;
        state.serialize_field(Self::map_key(tags::STUDY_DATE), &self.study_date)?;
        state.serialize_field(Self::map_key(tags::STUDY_TIME), &self.study_time)?;
        state.serialize_field(Self::map_key(tags::STUDY_DESCRIPTION), &self.study_desc)?;
        state.end()
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
            series_num: get_string(tags::SERIES_NUMBER, obj),
            series_uid: get_string(tags::SERIES_INSTANCE_UID, obj),
            series_date: get_string(tags::SERIES_DATE, obj),
            series_time: get_string(tags::SERIES_TIME, obj),
            series_desc: get_string(tags::SERIES_DESCRIPTION, obj),
            modality: get_string(tags::MODALITY, obj),
            body_part_exam: get_string(tags::BODY_PART_EXAMINED, obj),
            image_type: get_string(tags::IMAGE_TYPE, obj),
            accession_number: get_string(tags::ACCESSION_NUMBER, obj),
        }
    }
    fn map_key(tag: Tag) -> &'static str {
        match tag {
            tags::SERIES_NUMBER => "x00200011",
            tags::SERIES_INSTANCE_UID => "x0020000E",
            tags::SERIES_DATE => "x00080021",
            tags::SERIES_TIME => "x00080031",
            tags::SERIES_DESCRIPTION => "x0008103E",
            tags::MODALITY => "x00080060",
            tags::BODY_PART_EXAMINED => "x00080024",
            tags::IMAGE_TYPE => "x00080008",
            tags::ACCESSION_NUMBER => "x00080050",
            _ => "unknown",
        }
    }
}

impl Serialize for SeriesInfo {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = s.serialize_struct("SeriesInfo", 9)?;
        state.serialize_field(Self::map_key(tags::SERIES_NUMBER), &self.series_num)?;
        state.serialize_field(Self::map_key(tags::SERIES_INSTANCE_UID), &self.series_uid)?;
        state.serialize_field(Self::map_key(tags::SERIES_DATE), &self.series_date)?;
        state.serialize_field(Self::map_key(tags::SERIES_TIME), &self.series_time)?;
        state.serialize_field(Self::map_key(tags::SERIES_DESCRIPTION), &self.series_desc)?;
        state.serialize_field(Self::map_key(tags::MODALITY), &self.modality)?;
        state.serialize_field(
            Self::map_key(tags::BODY_PART_EXAMINED),
            &self.body_part_exam,
        )?;
        state.serialize_field(Self::map_key(tags::IMAGE_TYPE), &self.image_type)?;
        state.serialize_field(
            Self::map_key(tags::ACCESSION_NUMBER),
            &self.accession_number,
        )?;
        state.end()
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
            sop_uid: get_string(tags::SOP_INSTANCE_UID, obj),
            rows: get_tag_value(tags::ROWS, obj, 0),
            cols: get_tag_value(tags::COLUMNS, obj, 0),
            pixel_spacing: get_string(tags::PIXEL_SPACING, obj),
            slice_location: get_string(tags::SLICE_LOCATION, obj),
            slice_thickness: get_string(tags::SLICE_THICKNESS, obj),
            image_position: get_string(tags::IMAGE_POSITION_PATIENT, obj),
            image_orientation: get_string(tags::IMAGE_ORIENTATION_PATIENT, obj),
            window_center: get_string(tags::WINDOW_CENTER, obj),
            window_width: get_string(tags::WINDOW_WIDTH, obj),
            photometric_interpretation: get_string(tags::PHOTOMETRIC_INTERPRETATION, obj),
            bits_allocated: get_tag_value(tags::BITS_ALLOCATED, obj, 8),
            high_bit: get_tag_value(tags::HIGH_BIT, obj, 7),
            bit_stored: get_tag_value(tags::BITS_STORED, obj, 7),
            samples_per_pixel: get_tag_value(tags::SAMPLES_PER_PIXEL, obj, 1),
            pixel_representation: get_tag_value(tags::PIXEL_REPRESENTATION, obj, 1),
        }
    }

    fn map_key(tag: Tag) -> &'static str {
        match tag {
            tags::INSTANCE_NUMBER => "x00200013",
            tags::SOP_INSTANCE_UID => "x00080018",
            tags::ROWS => "x00280010",
            tags::COLUMNS => "x00280011",
            tags::PIXEL_SPACING => "x00280030",
            tags::SLICE_LOCATION => "x00201041",
            tags::SLICE_THICKNESS => "x00180050",
            tags::IMAGE_POSITION_PATIENT => "x00200032",
            tags::IMAGE_ORIENTATION_PATIENT => "x00200037",
            tags::WINDOW_CENTER => "x00281050",
            tags::WINDOW_WIDTH => "x00281051",
            tags::PHOTOMETRIC_INTERPRETATION => "x00280004",
            tags::BITS_ALLOCATED => "x00280100",
            tags::HIGH_BIT => "x00280101",
            tags::BITS_STORED => "x00280102",
            tags::SAMPLES_PER_PIXEL => "x00280002",
            tags::PIXEL_REPRESENTATION => "x00280103",
            _ => "unknown",
        }
    }
}
impl Serialize for ImageInfo {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = s.serialize_struct("ImageInfo", 17)?;
        state.serialize_field(ImageInfo::map_key(tags::INSTANCE_NUMBER), &self.inst_num)?;
        state.serialize_field(ImageInfo::map_key(tags::SOP_INSTANCE_UID), &self.sop_uid)?;
        state.serialize_field(ImageInfo::map_key(tags::ROWS), &self.rows)?;
        state.serialize_field(ImageInfo::map_key(tags::COLUMNS), &self.cols)?;
        state.serialize_field(ImageInfo::map_key(tags::PIXEL_SPACING), &self.pixel_spacing)?;
        state.serialize_field(
            ImageInfo::map_key(tags::SLICE_LOCATION),
            &self.slice_location,
        )?;
        state.serialize_field(
            ImageInfo::map_key(tags::SLICE_THICKNESS),
            &self.slice_thickness,
        )?;
        state.serialize_field(
            ImageInfo::map_key(tags::IMAGE_POSITION_PATIENT),
            &self.image_position,
        )?;
        state.serialize_field(
            ImageInfo::map_key(tags::IMAGE_ORIENTATION_PATIENT),
            &self.image_orientation,
        )?;
        state.serialize_field(ImageInfo::map_key(tags::WINDOW_CENTER), &self.window_center)?;
        state.serialize_field(ImageInfo::map_key(tags::WINDOW_WIDTH), &self.window_width)?;
        state.serialize_field(
            ImageInfo::map_key(tags::PHOTOMETRIC_INTERPRETATION),
            &self.photometric_interpretation,
        )?;
        state.serialize_field(
            ImageInfo::map_key(tags::BITS_ALLOCATED),
            &self.bits_allocated,
        )?;
        state.serialize_field(ImageInfo::map_key(tags::HIGH_BIT), &self.high_bit)?;
        state.serialize_field(ImageInfo::map_key(tags::BITS_STORED), &self.bit_stored)?;
        state.serialize_field(
            ImageInfo::map_key(tags::SAMPLES_PER_PIXEL),
            &self.samples_per_pixel,
        )?;
        state.serialize_field(
            ImageInfo::map_key(tags::PIXEL_REPRESENTATION),
            &self.pixel_representation,
        )?;
        state.end()
    }
}
