use std::fmt;
use clap::Parser;
use dicom::core::Tag;
use dicom::dictionary_std::tags;
use dicom::object::DefaultDicomObject;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeStruct;
use crate::dcmobj::get_string;

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
// 1. 定义所有字段的key枚举
#[derive(Debug)]
enum FieldPatientInfo {
    Id,
    Name,
    Age,
    Sex,
    BirthDate,
    BirthTime,
    Ignore,
}
impl<'de> Deserialize<'de> for FieldPatientInfo {
    fn deserialize<D>(deserializer: D) -> Result<FieldPatientInfo, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = FieldPatientInfo;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid PatientInfo field key")
            }

            fn visit_str<E>(self, value: &str) -> Result<FieldPatientInfo, E>
            where
                E: de::Error,
            {
                match value {
                    "x00100020" => Ok(FieldPatientInfo::Id),
                    "x00100010" => Ok(FieldPatientInfo::Name),
                    "x00101010" => Ok(FieldPatientInfo::Age),
                    "x00100040" => Ok(FieldPatientInfo::Sex),
                    "x00100030" => Ok(FieldPatientInfo::BirthDate),
                    "x00100032" => Ok(FieldPatientInfo::BirthTime),
                    _ => Ok(FieldPatientInfo::Ignore),
                }
            }
        }

        deserializer.deserialize_identifier(FieldVisitor)
    }
}
// 3. 实现Visitor
struct PatientInfoVisitor;

impl<'de> Visitor<'de> for PatientInfoVisitor {
    type Value = PatientInfo;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct PatientInfo")
    }

    fn visit_map<V>(self, mut map: V) -> Result<PatientInfo, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut id = None;
        let mut name = None;
        let mut age = None;
        let mut sex = None;
        let mut birth_date = None;
        let mut birth_time = None;
        while let Some(key) = map.next_key()? {
            match key {
                FieldPatientInfo::Id => { id = Some(map.next_value()?); }
                FieldPatientInfo::Name => { name = Some(map.next_value()?); }
                FieldPatientInfo::Age => { age = Some(map.next_value()?); }
                FieldPatientInfo::Sex => { sex = Some(map.next_value()?); }
                FieldPatientInfo::BirthDate => { birth_date = Some(map.next_value()?); }
                FieldPatientInfo::BirthTime => { birth_time = Some(map.next_value()?); }
                FieldPatientInfo::Ignore => { let _: de::IgnoredAny = map.next_value()?; }
            }
        }
        Ok(PatientInfo {
            id: id.ok_or_else(|| de::Error::missing_field("x00100020"))?,
            name: name.ok_or_else(|| de::Error::missing_field("x00100010"))?,
            age: age.ok_or_else(|| de::Error::missing_field("x00101010"))?,
            sex: sex.ok_or_else(|| de::Error::missing_field("x00100040"))?,
            birth_date: birth_date.ok_or_else(|| de::Error::missing_field("x00100030"))?,
            birth_time: birth_time.ok_or_else(|| de::Error::missing_field("x00100032"))?,
        })
    }
}

// 4. 实现Deserialize for PatientInfo
impl<'de> Deserialize<'de> for PatientInfo {
    fn deserialize<D>(deserializer: D) -> Result<PatientInfo, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &'static [&'static str] = &[
            "x00100020", "x00100010", "x00101010", "x00100040", "x00100030", "x00100032"
        ];
        deserializer.deserialize_struct("PatientInfo", FIELDS, PatientInfoVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn dummy_patient_info() -> PatientInfo {
        PatientInfo {
            id: "12345".to_string(),
            name: "张三".to_string(),
            age: "30Y".to_string(),
            sex: "M".to_string(),
            birth_date: "19950101".to_string(),
            birth_time: "120000".to_string(),
        }
    }

    #[test]
    fn test_serialize_patient_info() {
        let patient = dummy_patient_info();
        let json = serde_json::to_string(&patient).unwrap();
        // 这里的 key 要和 map_key 一致
        let expected_json = r#"{"x00100020":"12345","x00100010":"张三","x00101010":"30Y","x00100040":"M","x00100030":"19950101","x00100032":"120000"}"#;
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_deserialize_patient_info() {
        let json = r#"{"x00100020":"12345","x00100010":"张三","x00101010":"30Y","x00100040":"M","x00100030":"19950101","x00100032":"120000"}"#;
        let patient: PatientInfo = serde_json::from_str(json).unwrap();
        assert_eq!(patient.id, "12345");
        assert_eq!(patient.name, "张三");
        assert_eq!(patient.age, "30Y");
        assert_eq!(patient.sex, "M");
        assert_eq!(patient.birth_date, "19950101");
        assert_eq!(patient.birth_time, "120000");
    }
}