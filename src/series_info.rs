use std::fmt;
use clap::Parser;
use dicom::core::Tag;
use dicom::dictionary_std::tags;
use dicom::object::DefaultDicomObject;
use serde::{de, Deserializer, Serialize, Serializer};
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeStruct;
use crate::dcmobj::get_string;

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

 
// ===== Deserialize 实现 =====

#[derive(Debug)]
enum FieldSeriesInfo {
    SeriesNum,
    SeriesUid,
    SeriesDate,
    SeriesTime,
    SeriesDesc,
    Modality,
    BodyPartExam,
    ImageType,
    AccessionNumber,
    Ignore,
}

impl<'de> de::Deserialize<'de> for FieldSeriesInfo {
    fn deserialize<D>(deserializer: D) -> Result<FieldSeriesInfo, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = FieldSeriesInfo;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid SeriesInfo field key")
            }

            fn visit_str<E>(self, value: &str) -> Result<FieldSeriesInfo, E>
            where
                E: de::Error,
            {
                match value {
                    "x00200011" => Ok(FieldSeriesInfo::SeriesNum),
                    "x0020000E" => Ok(FieldSeriesInfo::SeriesUid),
                    "x00080021" => Ok(FieldSeriesInfo::SeriesDate),
                    "x00080031" => Ok(FieldSeriesInfo::SeriesTime),
                    "x0008103E" => Ok(FieldSeriesInfo::SeriesDesc),
                    "x00080060" => Ok(FieldSeriesInfo::Modality),
                    "x00080024" => Ok(FieldSeriesInfo::BodyPartExam),
                    "x00080008" => Ok(FieldSeriesInfo::ImageType),
                    "x00080050" => Ok(FieldSeriesInfo::AccessionNumber),
                    _ => Ok(FieldSeriesInfo::Ignore),
                }
            }
        }

        deserializer.deserialize_identifier(FieldVisitor)
    }
}

struct SeriesInfoVisitor;

impl<'de> Visitor<'de> for SeriesInfoVisitor {
    type Value = SeriesInfo;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct SeriesInfo")
    }

    fn visit_map<V>(self, mut map: V) -> Result<SeriesInfo, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut series_num = None;
        let mut series_uid = None;
        let mut series_date = None;
        let mut series_time = None;
        let mut series_desc = None;
        let mut modality = None;
        let mut body_part_exam = None;
        let mut image_type = None;
        let mut accession_number = None;

        while let Some(key) = map.next_key()? {
            match key {
                FieldSeriesInfo::SeriesNum => {
                    series_num = Some(map.next_value()?);
                }
                FieldSeriesInfo::SeriesUid => {
                    series_uid = Some(map.next_value()?);
                }
                FieldSeriesInfo::SeriesDate => {
                    series_date = Some(map.next_value()?);
                }
                FieldSeriesInfo::SeriesTime => {
                    series_time = Some(map.next_value()?);
                }
                FieldSeriesInfo::SeriesDesc => {
                    series_desc = Some(map.next_value()?);
                }
                FieldSeriesInfo::Modality => {
                    modality = Some(map.next_value()?);
                }
                FieldSeriesInfo::BodyPartExam => {
                    body_part_exam = Some(map.next_value()?);
                }
                FieldSeriesInfo::ImageType => {
                    image_type = Some(map.next_value()?);
                }
                FieldSeriesInfo::AccessionNumber => {
                    accession_number = Some(map.next_value()?);
                }
                FieldSeriesInfo::Ignore => {
                    let _: de::IgnoredAny = map.next_value()?;
                }
            }
        }

        Ok(SeriesInfo {
            series_num: series_num.ok_or_else(|| de::Error::missing_field("x00200011"))?,
            series_uid: series_uid.ok_or_else(|| de::Error::missing_field("x0020000E"))?,
            series_date: series_date.ok_or_else(|| de::Error::missing_field("x00080021"))?,
            series_time: series_time.ok_or_else(|| de::Error::missing_field("x00080031"))?,
            series_desc: series_desc.ok_or_else(|| de::Error::missing_field("x0008103E"))?,
            modality: modality.ok_or_else(|| de::Error::missing_field("x00080060"))?,
            body_part_exam: body_part_exam.ok_or_else(|| de::Error::missing_field("x00080024"))?,
            image_type: image_type.ok_or_else(|| de::Error::missing_field("x00080008"))?,
            accession_number: accession_number.ok_or_else(|| de::Error::missing_field("x00080050"))?,
        })
    }
}

impl<'de> de::Deserialize<'de> for SeriesInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &'static [&'static str] = &[
            "x00200011", // SERIES_NUMBER
            "x0020000E", // SERIES_INSTANCE_UID
            "x00080021", // SERIES_DATE
            "x00080031", // SERIES_TIME
            "x0008103E", // SERIES_DESCRIPTION
            "x00080060", // MODALITY
            "x00080024", // BODY_PART_EXAMINED
            "x00080008", // IMAGE_TYPE
            "x00080050", // ACCESSION_NUMBER
        ];

        deserializer.deserialize_struct("SeriesInfo", FIELDS, SeriesInfoVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn dummy_series_info() -> SeriesInfo {
        SeriesInfo {
            series_num: "1".to_string(),
            series_uid: "1.2.840.113619.2.1.20230101.123456.123".to_string(),
            series_date: "20230101".to_string(),
            series_time: "120000".to_string(),
            series_desc: "CT ABDOMEN".to_string(),
            modality: "CT".to_string(),
            body_part_exam: "ABDOMEN".to_string(),
            image_type: "ORIGINAL\\PRIMARY\\AXIAL".to_string(),
            accession_number: "ACC123456".to_string(),
        }
    }

    #[test]
    fn test_serialize_series_info() {
        let series = dummy_series_info();
        let json = serde_json::to_string(&series).unwrap();

        let expected_json = r#"
{
  "x00200011": "1",
  "x0020000E": "1.2.840.113619.2.1.20230101.123456.123",
  "x00080021": "20230101",
  "x00080031": "120000",
  "x0008103E": "CT ABDOMEN",
  "x00080060": "CT",
  "x00080024": "ABDOMEN",
  "x00080008": "ORIGINAL\\PRIMARY\\AXIAL",
  "x00080050": "ACC123456"
}"#
            .replace("\n", "")
            .replace(" ", "");

        let actual_json = json.replace(" ", "");
        assert_eq!(actual_json, expected_json);
    }

    #[test]
    fn test_deserialize_series_info() {
        let json = r#"
{
  "x00200011": "1",
  "x0020000E": "1.2.840.113619.2.1.20230101.123456.123",
  "x00080021": "20230101",
  "x00080031": "120000",
  "x0008103E": "CT ABDOMEN",
  "x00080060": "CT",
  "x00080024": "ABDOMEN",
  "x00080008": "ORIGINAL\\PRIMARY\\AXIAL",
  "x00080050": "ACC123456"
}"#;

        let series: SeriesInfo = serde_json::from_str(json).unwrap();

        assert_eq!(series.series_num, "1");
        assert_eq!(
            series.series_uid,
            "1.2.840.113619.2.1.20230101.123456.123"
        );
        assert_eq!(series.series_date, "20230101");
        assert_eq!(series.series_time, "120000");
        assert_eq!(series.series_desc, "CT ABDOMEN");
        assert_eq!(series.modality, "CT");
        assert_eq!(series.body_part_exam, "ABDOMEN");
        assert_eq!(series.image_type, "ORIGINAL\\PRIMARY\\AXIAL");
        assert_eq!(series.accession_number, "ACC123456");
    }
}
