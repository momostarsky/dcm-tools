use crate::dcmobj::get_string;
use clap::Parser;
use dicom::core::Tag;
use dicom::dictionary_std::tags;
use dicom::object::DefaultDicomObject;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
 
use serde::de::{self, MapAccess, Visitor};
use std::fmt;

#[derive(Debug)]
enum FieldStudyInfo {
    StudyId,
    StudyUid,
    StudyDate,
    StudyTime,
    StudyDesc,    
    Unknown,
}

impl<'de> Deserialize<'de> for FieldStudyInfo {
    fn deserialize<D>(deserializer: D) -> Result<FieldStudyInfo, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = FieldStudyInfo;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid PatientInfo field key")
            }

            fn visit_str<E>(self, value: &str) -> Result<FieldStudyInfo, E>
            where
                E: de::Error,
            {
                match value {
                    "x00200010" => Ok(FieldStudyInfo::StudyId),
                    "x0020000D" => Ok(FieldStudyInfo::StudyUid),
                    "x00080020" => Ok(FieldStudyInfo::StudyDate),
                    "x00080030" => Ok(FieldStudyInfo::StudyTime),
                    "x00081030" => Ok(FieldStudyInfo::StudyDesc),
                    _ => Ok(FieldStudyInfo::Unknown),
                }
            }
        }

        deserializer.deserialize_identifier(FieldVisitor)
    }
}
// 3. 实现Visitor
struct StudyInfoVisitor;

impl<'de> Visitor<'de> for StudyInfoVisitor {
    type Value = StudyInfo;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct StudyInfo")
    }
    fn visit_map<V>(self, mut map: V) -> Result<StudyInfo, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut id = None;
        let mut name = None;
        let mut age = None;
        let mut sex = None;
        let mut desc = None;

        while let Some(key) = map.next_key()? {
            match key {
                FieldStudyInfo::StudyId => {
                    id = Some(map.next_value()?);
                }
                FieldStudyInfo::StudyUid => {
                    name = Some(map.next_value()?);
                }
                FieldStudyInfo::StudyDate => {
                    age = Some(map.next_value()?);
                }
                FieldStudyInfo::StudyTime => {
                    sex = Some(map.next_value()?);
                }
                FieldStudyInfo::StudyDesc => {
                    desc = Some(map.next_value()?);
                }
                FieldStudyInfo::Unknown => {
                    map.next_value::<String>()?;
                }
            }
        }
        Ok(StudyInfo {
            study_id: id.ok_or_else(|| de::Error::missing_field("study_id"))?,
            study_uid: name.ok_or_else(|| de::Error::missing_field("study_uid"))?,
            study_date: age.ok_or_else(|| de::Error::missing_field("study_date"))?,
            study_time: sex.ok_or_else(|| de::Error::missing_field("study_time"))?,
            study_desc: desc.ok_or_else(|| de::Error::missing_field("study_desc"))?,
        })
    }
}
impl<'de> Deserialize<'de> for StudyInfo {
    fn deserialize<D>(deserializer: D) -> Result<StudyInfo, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &'static [&'static str] = &[
            "x00200010",
            "x0020000D",
            "x00080020",
            "x00080030",
            "x00081030",
        ];
        deserializer.deserialize_struct(" StudyInfo", FIELDS, StudyInfoVisitor)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn dummy_study_info() -> StudyInfo {
        StudyInfo {
            study_id: "12345".to_string(),
            study_uid: "1234567890".to_string(),
            study_date: "20240101".to_string(),
            study_time: "120103".to_string(),
            study_desc: "Study Description".to_string(),
        }
    }

    #[test]
    fn test_serialize_study_info() {
        let study = dummy_study_info();
        let json = serde_json::to_string(&study).unwrap();
        let expected_json = r#"{"x00200010":"12345","x0020000D":"1234567890","x00080020":"20240101","x00080030":"120103","x00081030":"Study Description"}"#; 
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_deserialize_study_info() {
        let json =  r#"{"x00200010":"12345","x0020000D":"1234567890","x00080020":"20240101","x00080030":"120103","x00081030":"Study Description"}"#;
        let study: StudyInfo = serde_json::from_str(json).unwrap();
        assert_eq!(study.study_id, "12345");
        assert_eq!(study.study_uid, "1234567890");
        assert_eq!(study.study_date, "20240101");
        assert_eq!(study.study_time, "120103");
        assert_eq!(study.study_desc, "Study Description");
    }
}