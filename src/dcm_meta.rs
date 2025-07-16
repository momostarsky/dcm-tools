use std::fmt;
use crate::dcmobj::get_string;
use clap::Parser;
use dicom_codegen::{DicomTagAccessors, DicomTagMapAccessors, TagMapAccessors};
use dicom_core::Tag;
use dicom_object::DefaultDicomObject;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{MapAccess, Visitor};
// use dicom::dictionary_std::tags;

#[derive(DicomTagAccessors, Debug, Parser, Clone)]
pub struct DcmMeta {
    pub transfer_syntax: String,
    #[dicom_tag(group(0x0008), element(0x0005))]
    pub character_set: String,
    #[dicom_tag(group(0x0008), element(0x0016))]
    pub sop_class_uid: String,
    #[dicom_tag(group(0x0008), element(0x0018))]
    pub sop_instance_uid: String,
    #[dicom_tag(group(0x0008), element(0x0008))]
    pub image_type: String,
}
impl DcmMeta {
    pub fn new(obj: &DefaultDicomObject) -> Self {
        Self {
            transfer_syntax: obj.meta().transfer_syntax().to_string(),
            character_set: get_string(DcmMeta::character_set_tag(), obj),
            sop_class_uid: get_string(DcmMeta::sop_class_uid_tag(), obj),
            sop_instance_uid: get_string(DcmMeta::sop_instance_uid_tag(), obj),
            image_type: get_string(DcmMeta::image_type_tag(), obj),
        }
    }
}

#[derive(TagMapAccessors, Debug, Parser, Clone)]
pub struct DcmMapMeta {
    #[map_tag(tag_name(0x0028, 0x0100))]
    pub bit_allocated: u16,
    #[map_tag(tag_name(0x0028, 0x0101))]
    pub bits_stored: u16,
    #[map_tag(tag_name(0x0028, 0x0102))]
    pub high_bit: u16,
}

impl DcmMapMeta {
    pub fn new(obj: &DefaultDicomObject) -> Self {
        Self {
            bit_allocated: obj.element(DcmMapMeta::bit_allocated_tag() ).unwrap().to_int().unwrap(),
            bits_stored: obj.element( DcmMapMeta::bits_stored_tag()  ).unwrap().to_int().unwrap(),
            high_bit: obj.element( DcmMapMeta::high_bit_tag()  ).unwrap().to_int().unwrap(),
        }
    }

    // 私有函数示例：验证位数是否合法
    fn validate_bits(&self) -> Result<(), String> {
        if self.bit_allocated <= 0 || self.bit_allocated > 32 {
            return Err("bit_allocated is between (0,32]".to_string());
        }
        if self.bit_allocated % 8 != 0 {
            return Err("bit_allocated must be a multiple of 8".to_string());
        }
        if self.bit_allocated < self.bits_stored {
            return Err("bit_allocated must be >= bits_stored".to_string());
        }

        if self.high_bit >= self.bits_stored {
            return Err("high_bit must be < bits_stored".to_string());
        }
        Ok(())
    }

    // 公共函数使用了私有函数
    pub fn check_valid(&self) -> bool {
        self.validate_bits().is_ok()
    }
}

// 工具函数：将 tag 转换为 "00XX,00YY" 格式字符串
fn tag_to_string(tag: Tag) -> String {
    format!("{:04X}{:04X}", tag.group(), tag.element())
}


#[derive(DicomTagMapAccessors, Debug, Parser, Clone)]
pub struct DcmEntityBaseMeta {
    #[map_tag_name(tag_name( dicom::dictionary_std::tags::BITS_ALLOCATED))]

    pub bit_allocated: u16,

    #[map_tag_name(tag_name( dicom::dictionary_std::tags::BITS_STORED ))]


    pub bits_stored: u16,

    #[map_tag_name(tag_name( dicom::dictionary_std::tags::HIGH_BIT))]

    pub high_bit: u16,



}


impl DcmEntityBaseMeta {
    pub fn new(obj: &DefaultDicomObject) -> Self {
        Self {
            bit_allocated: obj.element(DcmEntityBaseMeta::BIT_ALLOCATED_TAG).unwrap().to_int().unwrap(),
            bits_stored: obj.element(DcmEntityBaseMeta::BITS_STORED_TAG).unwrap().to_int().unwrap(),
            high_bit: obj.element(DcmEntityBaseMeta::HIGH_BIT_TAG).unwrap().to_int().unwrap(),

        }
    }
}

impl Serialize for DcmEntityBaseMeta {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry(&tag_to_string(DcmEntityBaseMeta::BIT_ALLOCATED_TAG), &self.bit_allocated)?;
        map.serialize_entry(&tag_to_string(DcmEntityBaseMeta::BITS_STORED_TAG), &self.bits_stored)?;
        map.serialize_entry(&tag_to_string(DcmEntityBaseMeta::HIGH_BIT_TAG), &self.high_bit)?;
        map.end()
    }
}
impl<'de> Deserialize<'de> for DcmEntityBaseMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>
    {
        struct MetaVisitor;

        impl<'de> Visitor<'de> for MetaVisitor {
            type Value = DcmEntityBaseMeta;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("DcmEntityBaseMeta with tag-keyed map")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where M: MapAccess<'de>
            {
                let mut bit_allocated = None;
                let mut bits_stored = None;
                let mut high_bit = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        k if k == tag_to_string(DcmEntityBaseMeta::BIT_ALLOCATED_TAG) => {
                            bit_allocated = Some(map.next_value()?);
                        }
                        k if k == tag_to_string(DcmEntityBaseMeta::BITS_STORED_TAG) => {
                            bits_stored = Some(map.next_value()?);
                        }
                        k if k == tag_to_string(DcmEntityBaseMeta::HIGH_BIT_TAG) => {
                            high_bit = Some(map.next_value()?);
                        }
                        _ => {
                            // 跳过未知字段
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(DcmEntityBaseMeta {
                    bit_allocated: bit_allocated.ok_or_else(|| de::Error::missing_field("bit_allocated"))?,
                    bits_stored: bits_stored.ok_or_else(|| de::Error::missing_field("bits_stored"))?,
                    high_bit: high_bit.ok_or_else(|| de::Error::missing_field("high_bit"))?,
                })
            }
        }

        deserializer.deserialize_map(MetaVisitor)
    }
}