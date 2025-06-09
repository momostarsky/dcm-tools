use crate::dcmobj::get_string;
use clap::Parser;
use dicom_codegen::{DicomTagAccessors, TagMapAccessors};
use dicom_core::Tag;
use dicom_object::DefaultDicomObject;
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
            bit_allocated: obj.element(Tag(0x0028, 0x0100)).unwrap().to_int().unwrap(),
            bits_stored: obj.element(Tag(0x0028, 0x0101)).unwrap().to_int().unwrap(),
            high_bit: obj.element(Tag(0x0028, 0x0102)).unwrap().to_int().unwrap(),
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
