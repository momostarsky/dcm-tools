use crate::dcmobj::get_string;
use crate::dcmobj::get_tag_value;
use clap::Parser;
use dicom::dictionary_std::tags;
use serde::{Deserialize, Serialize};
// use dicom_object::DefaultDicomObject;
// use dicom_core::{Tag, value::PrimitiveValue};
#[derive(Debug, Parser, Clone, Serialize, Deserialize)]
pub struct DicomInfo {

    pub transfer_syntax_uid: String,
    pub specific_character_set: String,
    pub sop_class_uid: String,
    pub sop_instance_uid: String,
    pub image_type: String,
    pub inst_num: u16,
    pub patient_id: String,
    pub patient_name: String,
    pub patient_age: String,
    pub patient_sex: String,
    pub patient_size: String,
    pub patient_weight: String,
    pub patient_birth_date: String,
    pub patient_birth_time: String,
    pub issuer_patient_id: String,

    pub study_id: String,
    pub study_uid: String,
    pub study_date: String,
    pub study_time: String,
    pub study_desc: String,

    pub series_num: String,
    pub series_uid: String,
    pub series_date: String,
    pub series_time: String,
    pub series_desc: String,
    pub modality: String,
    pub body_part_exam: String,
    pub accession_number: String,

    pub rows: u16,
    pub cols: u16,
    pub window_center: String,
    pub window_width: String,
    pub photometric_interpretation: String,
    pub bits_allocated: u16,
    pub high_bit: u16,
    pub bit_stored: u16,
    pub samples_per_pixel: u16,
    pub pixel_representation: u16,

    pub pixel_spacing: String,
    pub slice_location: String,
    pub slice_thickness: String,
    // 图像位置信息 Image Position (Patient)
    pub image_position: String,
    // 旋转信息 Image Orientation (Patient)
    pub image_orientation: String,

    pub rescale_slope: String,
    pub rescale_intercept: String,
    pub rescale_type: String,
}

impl DicomInfo {
    pub fn new(obj: &dicom::object::DefaultDicomObject) -> Self {
        Self {
            transfer_syntax_uid: obj.meta().transfer_syntax().to_string(),
            specific_character_set: get_string(tags::SPECIFIC_CHARACTER_SET, obj),
            sop_class_uid: get_string(tags::SOP_CLASS_UID, obj),
            sop_instance_uid: get_string(tags::SOP_INSTANCE_UID, obj),
            image_type: get_string(tags::IMAGE_TYPE, obj),
            inst_num: get_tag_value(tags::INSTANCE_NUMBER, obj, 0u16),
            patient_id: get_string(tags::PATIENT_ID, obj),
            patient_name: get_string(tags::PATIENT_NAME, obj),
            patient_age: get_string(tags::PATIENT_AGE, obj),
            patient_sex: get_string(tags::PATIENT_SEX, obj),
            patient_size: get_string(tags::PATIENT_SIZE, obj),
            patient_weight: get_string(tags::PATIENT_WEIGHT, obj),
            patient_birth_date: get_string(tags::PATIENT_BIRTH_DATE, obj),
            patient_birth_time: get_string(tags::PATIENT_BIRTH_TIME, obj),
            issuer_patient_id: get_string(tags::ISSUER_OF_PATIENT_ID, obj),
            study_id: get_string(tags::STUDY_ID, obj),
            study_uid: get_string(tags::STUDY_INSTANCE_UID, obj),
            study_date: get_string(tags::STUDY_DATE, obj),
            study_time: get_string(tags::STUDY_TIME, obj),
            study_desc: get_string(tags::STUDY_DESCRIPTION, obj),
            series_num: get_string(tags::SERIES_NUMBER, obj),
            series_uid: get_string(tags::SERIES_INSTANCE_UID, obj),
            series_date: get_string(tags::SERIES_DATE, obj),
            series_time: get_string(tags::SERIES_TIME, obj),
            series_desc: get_string(tags::SERIES_DESCRIPTION, obj),
            modality: get_string(tags::MODALITY, obj),
            body_part_exam: get_string(tags::BODY_PART_EXAMINED, obj),
            accession_number: get_string(tags::ACCESSION_NUMBER, obj),
            rows: get_tag_value(tags::ROWS, obj, 0u16),
            cols: get_tag_value(tags::COLUMNS, obj, 0u16),
            window_center: get_string(tags::WINDOW_CENTER, obj),
            window_width: get_string(tags::WINDOW_WIDTH, obj),
            photometric_interpretation: get_string(tags::PHOTOMETRIC_INTERPRETATION, obj),
            bits_allocated: get_tag_value(tags::BITS_ALLOCATED, obj, 0u16),
            high_bit: get_tag_value(tags::HIGH_BIT, obj, 0u16),
            bit_stored: get_tag_value(tags::BITS_STORED, obj, 0u16),
            samples_per_pixel: get_tag_value(tags::SAMPLES_PER_PIXEL, obj, 0u16),
            pixel_representation: get_tag_value(tags::PIXEL_REPRESENTATION, obj, 0u16),
            pixel_spacing: get_string(tags::PIXEL_SPACING, obj),
            slice_location: get_string(tags::SLICE_LOCATION, obj),
            slice_thickness: get_string(tags::SLICE_THICKNESS, obj),
            image_position: get_string(tags::IMAGE_POSITION_PATIENT, obj),
            image_orientation: get_string(tags::IMAGE_ORIENTATION_PATIENT, obj),
            rescale_slope: get_string(tags::RESCALE_SLOPE, obj),
            rescale_intercept: get_string(tags::RESCALE_INTERCEPT, obj),
            rescale_type: get_string(tags::RESCALE_TYPE, obj),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dicom::dictionary_std::uids;
    // use dicom_core::value::C;
    use dicom_object::{
        DefaultDicomObject, FileMetaTableBuilder, IMPLEMENTATION_CLASS_UID,
        IMPLEMENTATION_VERSION_NAME,
    };

    fn create_test_dicom_object() -> DefaultDicomObject {
        use dicom::core::{DataElement, PrimitiveValue, VR};
        use dicom::dictionary_std::tags;

        let sop_instance_uid = dicom_gen_uid::gen_uid();
        let meta = FileMetaTableBuilder::new()
            .media_storage_sop_class_uid(uids::CT_IMAGE_STORAGE)
            .media_storage_sop_instance_uid(sop_instance_uid)
            .transfer_syntax(
                dicom_transfer_syntax_registry::entries::EXPLICIT_VR_LITTLE_ENDIAN.uid(),
            )
            .implementation_class_uid(IMPLEMENTATION_CLASS_UID)
            .implementation_version_name(IMPLEMENTATION_VERSION_NAME)
            .build()
            .unwrap();

        let mut obj = DefaultDicomObject::new_empty_with_meta(meta);

        // 添加用于测试的字段
        obj.put(DataElement::new(
            tags::SPECIFIC_CHARACTER_SET,
            VR::CS,
            PrimitiveValue::from("ISO_IR 100"),
        ));
        obj.put(DataElement::new(
            tags::IMAGE_TYPE,
            VR::CS,
            PrimitiveValue::from("ORIGINAL\\PRIMARY\\AXIAL"),
        ));
        obj.put(DataElement::new(
            tags::INSTANCE_NUMBER,
            VR::IS,
            PrimitiveValue::from("123"),
        ));
        obj.put(DataElement::new(
            tags::PATIENT_ID,
            VR::LO,
            PrimitiveValue::from("P1234567"),
        ));
        obj.put(DataElement::new(
            tags::PATIENT_NAME,
            VR::PN,
            PrimitiveValue::from("Doe^John"),
        ));
        obj.put(DataElement::new(
            tags::PATIENT_AGE,
            VR::AS,
            PrimitiveValue::from("045Y"),
        ));
        obj.put(DataElement::new(
            tags::PATIENT_SEX,
            VR::CS,
            PrimitiveValue::from("M"),
        ));
        obj.put(DataElement::new(
            tags::STUDY_INSTANCE_UID,
            VR::UI,
            PrimitiveValue::from("1.2.3.4.5.6.7.8"),
        ));
        obj.put(DataElement::new(
            tags::SERIES_INSTANCE_UID,
            VR::UI,
            PrimitiveValue::from("1.2.3.4.5.6.7.8.1"),
        ));
        obj.put(DataElement::new(
            tags::ROWS,
            VR::US,
            PrimitiveValue::from(512u16),
        ));
        obj.put(DataElement::new(
            tags::COLUMNS,
            VR::US,
            PrimitiveValue::from(512u16),
        ));
        obj.put(DataElement::new(
            tags::WINDOW_CENTER,
            VR::DS,
            PrimitiveValue::from("40"),
        ));
        obj.put(DataElement::new(
            tags::WINDOW_WIDTH,
            VR::DS,
            PrimitiveValue::from("400"),
        ));
        obj.put(DataElement::new(
            tags::MODALITY,
            VR::CS,
            PrimitiveValue::from("CT"),
        ));
        obj.put(DataElement::new(
            tags::BODY_PART_EXAMINED,
            VR::CS,
            PrimitiveValue::from("CHEST"),
        ));
        //
        // let v = vec!["0.5".to_string(), "0.5".to_string()];
        // obj.put(DataElement::new(
        //     tags::PIXEL_SPACING,
        //     VR::DS,
        //     PrimitiveValue::Strs(<C<String>>::from(v)),
        // ));

        //推荐的方式
        let _c = vec!["0.5" , "0.5" ];
        obj.put(DataElement::new(
            tags::PIXEL_SPACING,
            VR::DS,
            PrimitiveValue::from(_c.join("\\")),
        ));




        obj.put(DataElement::new(
            tags::SLICE_THICKNESS,
            VR::DS,
            PrimitiveValue::from("5.0"),
        ));
        obj.put(DataElement::new(
            tags::RESCALE_SLOPE,
            VR::DS,
            PrimitiveValue::from("1.0"),
        ));
        obj.put(DataElement::new(
            tags::RESCALE_INTERCEPT,
            VR::DS,
            PrimitiveValue::from("-1000"),
        ));

        obj
    }

    #[test]
    fn test_serialize_deserialize() {
        let obj = create_test_dicom_object();
        let info = DicomInfo::new(&obj);
        let serialized = serde_json::to_string(&info).unwrap();
        let deserialized: DicomInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(info.image_type, deserialized.image_type);
        assert_eq!(info.patient_id, deserialized.patient_id);
        assert_eq!(info.patient_name, deserialized.patient_name);
        assert_eq!(info.patient_age, deserialized.patient_age);
        assert_eq!(info.patient_sex, deserialized.patient_sex);
        assert_eq!(info.study_uid, deserialized.study_uid);
        assert_eq!(info.series_uid, deserialized.series_uid);
        assert_eq!(info.rows, deserialized.rows);
        assert_eq!(info.cols, deserialized.cols);
        assert_eq!(info.window_center, deserialized.window_center);
        assert_eq!(info.window_width, deserialized.window_width);
        assert_eq!(info.modality, deserialized.modality);
        assert_eq!(info.body_part_exam, deserialized.body_part_exam);
        assert_eq!(info.pixel_spacing, deserialized.pixel_spacing);
        assert_eq!(info.slice_thickness, deserialized.slice_thickness);
        assert_eq!(info.rescale_slope, deserialized.rescale_slope);
        assert_eq!(info.rescale_intercept, deserialized.rescale_intercept);
    }
}
