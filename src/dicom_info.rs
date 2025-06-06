use crate::dcmobj::{get_string, json_key};
use clap::Parser;
use std::fmt;
// use clap::Parser;
use dicom::dictionary_std::tags;
use dicom_object::DefaultDicomObject;
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
// use dicom_object::DefaultDicomObject;
// use dicom_core::{Tag, value::PrimitiveValue};
// use dicom_macros::dicom;

#[derive(Debug, Parser, Clone)]
pub struct DicomInfo {
    pub transfer_syntax_uid: String,

    pub specific_character_set: String,

    pub sop_class_uid: String,

    pub sop_instance_uid: String,

    pub image_type: String,

    pub study_uid: String,

    pub series_uid: String,

    pub patient_id: String,

    pub patient_name: String,

    pub patient_sex: String,

    pub patient_age: String,

    pub patient_size: String,

    pub patient_weight: String,

    pub patient_birth_date: String,

    pub patient_birth_time: String,

    pub patient_ethnic_group: String,

    pub patient_occupation: String,

    pub instance_number: String,

    pub rows: String,

    pub columns: String,

    pub pixel_spacing: String,

    pub slice_thickness: String,

    pub slice_location: String,

    pub window_center: String,

    pub window_width: String,

    pub rescale_intercept: String,

    pub rescale_slope: String,

    pub rescale_type: String,

    pub image_position_patient: String,

    pub image_orientation_patient: String,

    pub bits_allocated: String,

    pub bits_stored: String,

    pub high_bit: String,

    pub pixel_representation: String,

    pub samples_per_pixel: String,

    pub photometric_interpretation: String,

    modality: String,

    body_part_examined: String,

    series_description: String,

    series_number: String,

    pub series_date: String,

    pub series_time: String,

    study_description: String,

    study_id: String,

    pub study_date: String,

    pub study_time: String,

    pub accession_number: String,
}

impl DicomInfo {
    pub fn new(obj: &DefaultDicomObject) -> Self {
        Self {
            transfer_syntax_uid: obj.meta().transfer_syntax().to_string(),
            sop_class_uid: get_string(tags::SOP_CLASS_UID, obj),
            sop_instance_uid: get_string(tags::SOP_INSTANCE_UID, obj),
            specific_character_set: get_string(tags::SPECIFIC_CHARACTER_SET, obj),
            image_type: get_string(tags::IMAGE_TYPE, obj),
            study_uid: get_string(tags::STUDY_INSTANCE_UID, obj),
            series_uid: get_string(tags::SERIES_INSTANCE_UID, obj),
            patient_id: get_string(tags::PATIENT_ID, obj),
            patient_name: get_string(tags::PATIENT_NAME, obj),
            patient_sex: get_string(tags::PATIENT_SEX, obj),
            patient_age: get_string(tags::PATIENT_AGE, obj),
            patient_size: get_string(tags::PATIENT_SIZE, obj),
            patient_weight: get_string(tags::PATIENT_WEIGHT, obj),
            patient_birth_date: get_string(tags::PATIENT_BIRTH_DATE, obj),
            patient_birth_time: get_string(tags::PATIENT_BIRTH_TIME, obj),
            patient_ethnic_group: get_string(tags::ETHNIC_GROUP, obj),
            patient_occupation: get_string(tags::OCCUPATION, obj),
            instance_number: get_string(tags::INSTANCE_NUMBER, obj),
            rows: get_string(tags::ROWS, obj),
            columns: get_string(tags::COLUMNS, obj),
            pixel_spacing: get_string(tags::PIXEL_SPACING, obj),
            slice_location: get_string(tags::SLICE_LOCATION, obj),
            slice_thickness: get_string(tags::SLICE_THICKNESS, obj),
            window_center: get_string(tags::WINDOW_CENTER, obj),
            window_width: get_string(tags::WINDOW_WIDTH, obj),
            rescale_intercept: get_string(tags::RESCALE_INTERCEPT, obj),
            rescale_slope: get_string(tags::RESCALE_SLOPE, obj),
            rescale_type: get_string(tags::RESCALE_TYPE, obj),

            image_position_patient: get_string(tags::IMAGE_POSITION_PATIENT, obj),
            image_orientation_patient: get_string(tags::IMAGE_ORIENTATION_PATIENT, obj),
            bits_allocated: get_string(tags::BITS_ALLOCATED, obj),
            bits_stored: get_string(tags::BITS_STORED, obj),
            high_bit: get_string(tags::HIGH_BIT, obj),
            pixel_representation: get_string(tags::PIXEL_REPRESENTATION, obj),
            samples_per_pixel: get_string(tags::SAMPLES_PER_PIXEL, obj),
            photometric_interpretation: get_string(tags::PHOTOMETRIC_INTERPRETATION, obj),
            modality: get_string(tags::MODALITY, obj),
            body_part_examined: get_string(tags::BODY_PART_EXAMINED, obj),
            series_description: get_string(tags::SERIES_DESCRIPTION, obj),
            series_number: get_string(tags::SERIES_NUMBER, obj),
            series_date: get_string(tags::SERIES_DATE, obj),
            series_time: get_string(tags::SERIES_TIME, obj),

            study_description: get_string(tags::STUDY_DESCRIPTION, obj),
            study_id: get_string(tags::STUDY_ID, obj),
            study_date: get_string(tags::STUDY_DATE, obj),
            study_time: get_string(tags::STUDY_TIME, obj),
            accession_number: get_string(tags::ACCESSION_NUMBER, obj),
        }
    }
}

impl Serialize for DicomInfo {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry(
            &json_key(tags::TRANSFER_SYNTAX_UID),
            &self.transfer_syntax_uid,
        )?;
        map.serialize_entry(&json_key(tags::SOP_CLASS_UID), &self.sop_class_uid)?;
        map.serialize_entry(&json_key(tags::SOP_INSTANCE_UID), &self.sop_instance_uid)?;
        map.serialize_entry(
            &json_key(tags::SPECIFIC_CHARACTER_SET),
            &self.specific_character_set,
        )?;
        map.serialize_entry(&json_key(tags::IMAGE_TYPE), &self.image_type)?;
        map.serialize_entry(&json_key(tags::STUDY_INSTANCE_UID), &self.study_uid)?;
        map.serialize_entry(&json_key(tags::SERIES_INSTANCE_UID), &self.series_uid)?;
        map.serialize_entry(&json_key(tags::PATIENT_ID), &self.patient_id)?;
        map.serialize_entry(&json_key(tags::PATIENT_NAME), &self.patient_name)?;
        map.serialize_entry(&json_key(tags::PATIENT_SEX), &self.patient_sex)?;
        map.serialize_entry(&json_key(tags::PATIENT_AGE), &self.patient_age)?;
        map.serialize_entry(
            &json_key(tags::PATIENT_BIRTH_DATE),
            &self.patient_birth_date,
        )?;
        map.serialize_entry(
            &json_key(tags::PATIENT_BIRTH_TIME),
            &self.patient_birth_time,
        )?;
        map.serialize_entry(&json_key(tags::PATIENT_SIZE), &self.patient_size)?;
        map.serialize_entry(&json_key(tags::PATIENT_WEIGHT), &self.patient_weight)?;
        map.serialize_entry(&json_key(tags::ETHNIC_GROUP), &self.patient_ethnic_group)?;
        map.serialize_entry(&json_key(tags::OCCUPATION), &self.patient_occupation)?;
        map.serialize_entry(&json_key(tags::INSTANCE_NUMBER), &self.instance_number)?;
        map.serialize_entry(&json_key(tags::ROWS), &self.rows)?;
        map.serialize_entry(&json_key(tags::COLUMNS), &self.columns)?;
        map.serialize_entry(&json_key(tags::PIXEL_SPACING), &self.pixel_spacing)?;
        map.serialize_entry(&json_key(tags::SLICE_THICKNESS), &self.slice_thickness)?;
        map.serialize_entry(&json_key(tags::SLICE_LOCATION), &self.slice_location)?;
        map.serialize_entry(&json_key(tags::WINDOW_CENTER), &self.window_center)?;
        map.serialize_entry(&json_key(tags::WINDOW_WIDTH), &self.window_width)?;
        map.serialize_entry(&json_key(tags::RESCALE_INTERCEPT), &self.rescale_intercept)?;
        map.serialize_entry(&json_key(tags::RESCALE_SLOPE), &self.rescale_slope)?;
        map.serialize_entry(&json_key(tags::RESCALE_TYPE), &self.rescale_type)?;
        map.serialize_entry(
            &json_key(tags::IMAGE_POSITION_PATIENT),
            &self.image_position_patient,
        )?;
        map.serialize_entry(
            &json_key(tags::IMAGE_ORIENTATION_PATIENT),
            &self.image_orientation_patient,
        )?;
        map.serialize_entry(&json_key(tags::BITS_ALLOCATED), &self.bits_allocated)?;
        map.serialize_entry(&json_key(tags::BITS_STORED), &self.bits_stored)?;
        map.serialize_entry(&json_key(tags::HIGH_BIT), &self.high_bit)?;
        map.serialize_entry(
            &json_key(tags::PIXEL_REPRESENTATION),
            &self.pixel_representation,
        )?;
        map.serialize_entry(&json_key(tags::SAMPLES_PER_PIXEL), &self.samples_per_pixel)?;
        map.serialize_entry(
            &json_key(tags::PHOTOMETRIC_INTERPRETATION),
            &self.photometric_interpretation,
        )?;
        map.serialize_entry(&json_key(tags::MODALITY), &self.modality)?;
        map.serialize_entry(
            &json_key(tags::BODY_PART_EXAMINED),
            &self.body_part_examined,
        )?;
        map.serialize_entry(
            &json_key(tags::SERIES_DESCRIPTION),
            &self.series_description,
        )?;
        map.serialize_entry(&json_key(tags::SERIES_NUMBER), &self.series_number)?;
        map.serialize_entry(&json_key(tags::SERIES_DATE), &self.series_date)?;
        map.serialize_entry(&json_key(tags::SERIES_TIME), &self.series_time)?;
        map.serialize_entry(&json_key(tags::STUDY_DESCRIPTION), &self.study_description)?;
        map.serialize_entry(&json_key(tags::STUDY_ID), &self.study_id)?;
        map.serialize_entry(&json_key(tags::STUDY_DATE), &self.study_date)?;
        map.serialize_entry(&json_key(tags::STUDY_TIME), &self.study_time)?;
        map.serialize_entry(&json_key(tags::ACCESSION_NUMBER), &self.accession_number)?;
        map.end()
    }
}

impl<'de> Deserialize<'de> for DicomInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DicomInfoVisitor;

        impl<'de> Visitor<'de> for DicomInfoVisitor {
            type Value = DicomInfo;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("DicomInfo struct with DICOM tag keys")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DicomInfo, V::Error>
            where
                V: MapAccess<'de>,
            {
                use crate::dcmobj::json_key;
                use dicom::dictionary_std::tags;
                use serde::de;

                // 初始化所有字段为 None
                let mut fields = std::collections::HashMap::new();
                fields.insert(json_key(tags::TRANSFER_SYNTAX_UID), None);
                fields.insert(json_key(tags::SPECIFIC_CHARACTER_SET), None);
                fields.insert(json_key(tags::SOP_CLASS_UID), None);
                fields.insert(json_key(tags::SOP_INSTANCE_UID), None);
                fields.insert(json_key(tags::IMAGE_TYPE), None);
                fields.insert(json_key(tags::STUDY_INSTANCE_UID), None);
                fields.insert(json_key(tags::SERIES_INSTANCE_UID), None);
                fields.insert(json_key(tags::PATIENT_ID), None);
                fields.insert(json_key(tags::PATIENT_NAME), None);
                fields.insert(json_key(tags::PATIENT_SEX), None);
                fields.insert(json_key(tags::PATIENT_AGE), None);
                fields.insert(json_key(tags::PATIENT_SIZE), None);
                fields.insert(json_key(tags::PATIENT_WEIGHT), None);
                fields.insert(json_key(tags::PATIENT_BIRTH_DATE), None);
                fields.insert(json_key(tags::PATIENT_BIRTH_TIME), None);
                fields.insert(json_key(tags::ETHNIC_GROUP), None);
                fields.insert(json_key(tags::OCCUPATION), None);
                fields.insert(json_key(tags::INSTANCE_NUMBER), None);
                fields.insert(json_key(tags::ROWS), None);
                fields.insert(json_key(tags::COLUMNS), None);
                fields.insert(json_key(tags::PIXEL_SPACING), None);
                fields.insert(json_key(tags::SLICE_THICKNESS), None);
                fields.insert(json_key(tags::SLICE_LOCATION), None);
                fields.insert(json_key(tags::WINDOW_CENTER), None);
                fields.insert(json_key(tags::WINDOW_WIDTH), None);
                fields.insert(json_key(tags::RESCALE_INTERCEPT), None);
                fields.insert(json_key(tags::RESCALE_SLOPE), None);
                fields.insert(json_key(tags::RESCALE_TYPE), None);
                fields.insert(json_key(tags::IMAGE_POSITION_PATIENT), None);
                fields.insert(json_key(tags::IMAGE_ORIENTATION_PATIENT), None);
                fields.insert(json_key(tags::BITS_ALLOCATED), None);
                fields.insert(json_key(tags::BITS_STORED), None);
                fields.insert(json_key(tags::HIGH_BIT), None);
                fields.insert(json_key(tags::PIXEL_REPRESENTATION), None);
                fields.insert(json_key(tags::SAMPLES_PER_PIXEL), None);
                fields.insert(json_key(tags::PHOTOMETRIC_INTERPRETATION), None);
                fields.insert(json_key(tags::MODALITY), None);
                fields.insert(json_key(tags::BODY_PART_EXAMINED), None);
                fields.insert(json_key(tags::SERIES_DESCRIPTION), None);
                fields.insert(json_key(tags::SERIES_NUMBER), None);
                fields.insert(json_key(tags::SERIES_DATE), None);
                fields.insert(json_key(tags::SERIES_TIME), None);
                fields.insert(json_key(tags::STUDY_DESCRIPTION), None);
                fields.insert(json_key(tags::STUDY_ID), None);
                fields.insert(json_key(tags::STUDY_DATE), None);
                fields.insert(json_key(tags::STUDY_TIME), None);
                fields.insert(json_key(tags::ACCESSION_NUMBER), None);

                while let Some(key) = map.next_key::<String>()? {
                    if let Some(entry) = fields.get_mut(&key) {
                        if entry.is_some() {
                            return Err(de::Error::custom(format!("missing field: {}", key)));
                        }
                        *entry = Some(map.next_value()?);
                    } else {
                        let _: de::IgnoredAny = map.next_value()?;
                    }
                }

                // 获取所有字段的值，若缺失则报错
                let get_field = |tag| {
                    let key = json_key(tag); // 保存到变量，生命周期延长
                    fields
                        .get(&key)
                        .and_then(|v| v.clone())
                        .ok_or_else(|| de::Error::custom(format!("missing field: {}", key)))
                };

                Ok(DicomInfo {
                    transfer_syntax_uid: get_field(tags::TRANSFER_SYNTAX_UID)?,
                    specific_character_set: get_field(tags::SPECIFIC_CHARACTER_SET)?,
                    sop_class_uid: get_field(tags::SOP_CLASS_UID)?,
                    sop_instance_uid: get_field(tags::SOP_INSTANCE_UID)?,
                    image_type: get_field(tags::IMAGE_TYPE)?,
                    study_uid: get_field(tags::STUDY_INSTANCE_UID)?,
                    series_uid: get_field(tags::SERIES_INSTANCE_UID)?,
                    patient_id: get_field(tags::PATIENT_ID)?,
                    patient_name: get_field(tags::PATIENT_NAME)?,
                    patient_sex: get_field(tags::PATIENT_SEX)?,
                    patient_age: get_field(tags::PATIENT_AGE)?,
                    patient_size: get_field(tags::PATIENT_SIZE)?,
                    patient_weight: get_field(tags::PATIENT_WEIGHT)?,
                    patient_birth_date: get_field(tags::PATIENT_BIRTH_DATE)?,
                    patient_birth_time: get_field(tags::PATIENT_BIRTH_TIME)?,
                    patient_ethnic_group: get_field(tags::ETHNIC_GROUP)?,
                    patient_occupation: get_field(tags::OCCUPATION)?,
                    instance_number: get_field(tags::INSTANCE_NUMBER)?,
                    rows: get_field(tags::ROWS)?,
                    columns: get_field(tags::COLUMNS)?,
                    pixel_spacing: get_field(tags::PIXEL_SPACING)?,
                    slice_thickness: get_field(tags::SLICE_THICKNESS)?,
                    slice_location: get_field(tags::SLICE_LOCATION)?,
                    window_center: get_field(tags::WINDOW_CENTER)?,
                    window_width: get_field(tags::WINDOW_WIDTH)?,
                    rescale_intercept: get_field(tags::RESCALE_INTERCEPT)?,
                    rescale_slope: get_field(tags::RESCALE_SLOPE)?,
                    rescale_type: get_field(tags::RESCALE_TYPE)?,
                    image_position_patient: get_field(tags::IMAGE_POSITION_PATIENT)?,
                    image_orientation_patient: get_field(tags::IMAGE_ORIENTATION_PATIENT)?,
                    bits_allocated: get_field(tags::BITS_ALLOCATED)?,
                    bits_stored: get_field(tags::BITS_STORED)?,
                    high_bit: get_field(tags::HIGH_BIT)?,
                    pixel_representation: get_field(tags::PIXEL_REPRESENTATION)?,
                    samples_per_pixel: get_field(tags::SAMPLES_PER_PIXEL)?,
                    photometric_interpretation: get_field(tags::PHOTOMETRIC_INTERPRETATION)?,
                    modality: get_field(tags::MODALITY)?,
                    body_part_examined: get_field(tags::BODY_PART_EXAMINED)?,
                    series_description: get_field(tags::SERIES_DESCRIPTION)?,
                    series_number: get_field(tags::SERIES_NUMBER)?,
                    series_date: get_field(tags::SERIES_DATE)?,
                    series_time: get_field(tags::SERIES_TIME)?,
                    study_description: get_field(tags::STUDY_DESCRIPTION)?,
                    study_id: get_field(tags::STUDY_ID)?,
                    study_date: get_field(tags::STUDY_DATE)?,
                    study_time: get_field(tags::STUDY_TIME)?,
                    accession_number: get_field(tags::ACCESSION_NUMBER)?,
                })
            }
        }

        deserializer.deserialize_map(DicomInfoVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde_json;
    use dicom::dictionary_std::uids;
    // use dicom_core::value::C;
    use crate::dicom_info::DicomInfo;
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
        let _c = vec!["0.5", "0.5"];
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


    fn create_dicom_from_file()->DefaultDicomObject{
        let dicom_file = "./t1.dcm";
         dicom::object::open_file(dicom_file).unwrap()

    }
    #[test]
    fn dicom_info_test_serialize_deserialize_all_fields() {
        let obj = create_test_dicom_object();
        let info = DicomInfo::new(&obj);

        // 序列化
        let serialized = serde_json::to_string(&info).unwrap();



        // 反序列化
        let deserialized: DicomInfo = serde_json::from_str(&serialized).unwrap();

        // 验证字段一致性
        assert_eq!(info.transfer_syntax_uid, deserialized.transfer_syntax_uid);
        assert_eq!(
            info.specific_character_set,
            deserialized.specific_character_set
        );
        assert_eq!(info.sop_class_uid, deserialized.sop_class_uid);
        assert_eq!(info.sop_instance_uid, deserialized.sop_instance_uid);
        assert_eq!(info.image_type, deserialized.image_type);
        assert_eq!(info.study_uid, deserialized.study_uid);
        assert_eq!(info.series_uid, deserialized.series_uid);
        assert_eq!(info.patient_id, deserialized.patient_id);
        assert_eq!(info.patient_name, deserialized.patient_name);
        assert_eq!(info.patient_sex, deserialized.patient_sex);
        assert_eq!(info.patient_age, deserialized.patient_age);
        assert_eq!(info.patient_size, deserialized.patient_size);
        assert_eq!(info.patient_weight, deserialized.patient_weight);
        assert_eq!(info.patient_birth_date, deserialized.patient_birth_date);
        assert_eq!(info.patient_birth_time, deserialized.patient_birth_time);
        assert_eq!(info.patient_ethnic_group, deserialized.patient_ethnic_group);
        assert_eq!(info.patient_occupation, deserialized.patient_occupation);
        assert_eq!(info.instance_number, deserialized.instance_number);
        assert_eq!(info.rows, deserialized.rows);
        assert_eq!(info.columns, deserialized.columns);
        assert_eq!(info.pixel_spacing, deserialized.pixel_spacing);
        assert_eq!(info.slice_thickness, deserialized.slice_thickness);
        assert_eq!(info.slice_location, deserialized.slice_location);
        assert_eq!(info.window_center, deserialized.window_center);
        assert_eq!(info.window_width, deserialized.window_width);
        assert_eq!(info.rescale_intercept, deserialized.rescale_intercept);
        assert_eq!(info.rescale_slope, deserialized.rescale_slope);
        assert_eq!(info.rescale_type, deserialized.rescale_type);
        assert_eq!(
            info.image_position_patient,
            deserialized.image_position_patient
        );
        assert_eq!(
            info.image_orientation_patient,
            deserialized.image_orientation_patient
        );
        assert_eq!(info.bits_allocated, deserialized.bits_allocated);
        assert_eq!(info.bits_stored, deserialized.bits_stored);
        assert_eq!(info.high_bit, deserialized.high_bit);
        assert_eq!(info.pixel_representation, deserialized.pixel_representation);
        assert_eq!(info.samples_per_pixel, deserialized.samples_per_pixel);
        assert_eq!(
            info.photometric_interpretation,
            deserialized.photometric_interpretation
        );
        assert_eq!(info.modality, deserialized.modality);
        assert_eq!(info.body_part_examined, deserialized.body_part_examined);
        assert_eq!(info.series_description, deserialized.series_description);
        assert_eq!(info.series_number, deserialized.series_number);
        assert_eq!(info.series_date, deserialized.series_date);
        assert_eq!(info.series_time, deserialized.series_time);
        assert_eq!(info.study_description, deserialized.study_description);
        assert_eq!(info.study_id, deserialized.study_id);
        assert_eq!(info.study_date, deserialized.study_date);
        assert_eq!(info.study_time, deserialized.study_time);
        assert_eq!(info.accession_number, deserialized.accession_number);
    }

    #[test]
    fn dicom_info_test_serialize_deserialize_all_fields2() {
        let obj = create_dicom_from_file();
        let info = DicomInfo::new(&obj);

        // 序列化
        let serialized = serde_json::to_string(&info).unwrap();



        // 反序列化
        let deserialized: DicomInfo = serde_json::from_str(&serialized).unwrap();

        // 验证字段一致性
        assert_eq!(info.transfer_syntax_uid, deserialized.transfer_syntax_uid);
        assert_eq!(
            info.specific_character_set,
            deserialized.specific_character_set
        );
        assert_eq!(info.sop_class_uid, deserialized.sop_class_uid);
        assert_eq!(info.sop_instance_uid, deserialized.sop_instance_uid);
        assert_eq!(info.image_type, deserialized.image_type);
        assert_eq!(info.study_uid, deserialized.study_uid);
        assert_eq!(info.series_uid, deserialized.series_uid);
        assert_eq!(info.patient_id, deserialized.patient_id);
        assert_eq!(info.patient_name, deserialized.patient_name);
        assert_eq!(info.patient_sex, deserialized.patient_sex);
        assert_eq!(info.patient_age, deserialized.patient_age);
        assert_eq!(info.patient_size, deserialized.patient_size);
        assert_eq!(info.patient_weight, deserialized.patient_weight);
        assert_eq!(info.patient_birth_date, deserialized.patient_birth_date);
        assert_eq!(info.patient_birth_time, deserialized.patient_birth_time);
        assert_eq!(info.patient_ethnic_group, deserialized.patient_ethnic_group);
        assert_eq!(info.patient_occupation, deserialized.patient_occupation);
        assert_eq!(info.instance_number, deserialized.instance_number);
        assert_eq!(info.rows, deserialized.rows);
        assert_eq!(info.columns, deserialized.columns);
        assert_eq!(info.pixel_spacing, deserialized.pixel_spacing);
        assert_eq!(info.slice_thickness, deserialized.slice_thickness);
        assert_eq!(info.slice_location, deserialized.slice_location);
        assert_eq!(info.window_center, deserialized.window_center);
        assert_eq!(info.window_width, deserialized.window_width);
        assert_eq!(info.rescale_intercept, deserialized.rescale_intercept);
        assert_eq!(info.rescale_slope, deserialized.rescale_slope);
        assert_eq!(info.rescale_type, deserialized.rescale_type);
        assert_eq!(
            info.image_position_patient,
            deserialized.image_position_patient
        );
        assert_eq!(
            info.image_orientation_patient,
            deserialized.image_orientation_patient
        );
        assert_eq!(info.bits_allocated, deserialized.bits_allocated);
        assert_eq!(info.bits_stored, deserialized.bits_stored);
        assert_eq!(info.high_bit, deserialized.high_bit);
        assert_eq!(info.pixel_representation, deserialized.pixel_representation);
        assert_eq!(info.samples_per_pixel, deserialized.samples_per_pixel);
        assert_eq!(
            info.photometric_interpretation,
            deserialized.photometric_interpretation
        );
        assert_eq!(info.modality, deserialized.modality);
        assert_eq!(info.body_part_examined, deserialized.body_part_examined);
        assert_eq!(info.series_description, deserialized.series_description);
        assert_eq!(info.series_number, deserialized.series_number);
        assert_eq!(info.series_date, deserialized.series_date);
        assert_eq!(info.series_time, deserialized.series_time);
        assert_eq!(info.study_description, deserialized.study_description);
        assert_eq!(info.study_id, deserialized.study_id);
        assert_eq!(info.study_date, deserialized.study_date);
        assert_eq!(info.study_time, deserialized.study_time);
        assert_eq!(info.accession_number, deserialized.accession_number);
    }
}
