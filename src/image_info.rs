use std::fmt;
use clap::Parser;
use dicom::core::Tag;
use dicom::dictionary_std::tags;
use dicom::object::DefaultDicomObject;
use serde::{de, Deserializer, Serialize, Serializer};
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeStruct;
use crate::dcmobj::{get_string, get_tag_value};


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
 
// ===== Deserialize 实现 =====

#[derive(Debug)]
enum FieldImageInfo {
    InstNum,
    SopUid,
    Rows,
    Cols,
    PixelSpacing,
    SliceLocation,
    SliceThickness,
    ImagePosition,
    ImageOrientation,
    WindowCenter,
    WindowWidth,
    PhotometricInterpretation,
    BitsAllocated,
    HighBit,
    BitStored,
    SamplesPerPixel,
    PixelRepresentation,
    Ignore,
}

impl<'de> de::Deserialize<'de> for FieldImageInfo {
    fn deserialize<D>(deserializer: D) -> Result<FieldImageInfo, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = FieldImageInfo;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid ImageInfo field key")
            }

            fn visit_str<E>(self, value: &str) -> Result<FieldImageInfo, E>
            where
                E: de::Error,
            {
                match value {
                    "x00200013" => Ok(FieldImageInfo::InstNum),
                    "x00080018" => Ok(FieldImageInfo::SopUid),
                    "x00280010" => Ok(FieldImageInfo::Rows),
                    "x00280011" => Ok(FieldImageInfo::Cols),
                    "x00280030" => Ok(FieldImageInfo::PixelSpacing),
                    "x00201041" => Ok(FieldImageInfo::SliceLocation),
                    "x00180050" => Ok(FieldImageInfo::SliceThickness),
                    "x00200032" => Ok(FieldImageInfo::ImagePosition),
                    "x00200037" => Ok(FieldImageInfo::ImageOrientation),
                    "x00281050" => Ok(FieldImageInfo::WindowCenter),
                    "x00281051" => Ok(FieldImageInfo::WindowWidth),
                    "x00280004" => Ok(FieldImageInfo::PhotometricInterpretation),
                    "x00280100" => Ok(FieldImageInfo::BitsAllocated),
                    "x00280101" => Ok(FieldImageInfo::HighBit),
                    "x00280102" => Ok(FieldImageInfo::BitStored),
                    "x00280002" => Ok(FieldImageInfo::SamplesPerPixel),
                    "x00280103" => Ok(FieldImageInfo::PixelRepresentation),
                    _ => Ok(FieldImageInfo::Ignore),
                }
            }
        }

        deserializer.deserialize_identifier(FieldVisitor)
    }
}

struct ImageInfoVisitor;

impl<'de> Visitor<'de> for ImageInfoVisitor {
    type Value = ImageInfo;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct ImageInfo")
    }

    fn visit_map<V>(self, mut map: V) -> Result<ImageInfo, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut inst_num = None;
        let mut sop_uid = None;
        let mut rows = None;
        let mut cols = None;
        let mut pixel_spacing = None;
        let mut slice_location = None;
        let mut slice_thickness = None;
        let mut image_position = None;
        let mut image_orientation = None;
        let mut window_center = None;
        let mut window_width = None;
        let mut photometric_interpretation = None;
        let mut bits_allocated = None;
        let mut high_bit = None;
        let mut bit_stored = None;
        let mut samples_per_pixel = None;
        let mut pixel_representation = None;

        while let Some(key) = map.next_key()? {
            match key {
                FieldImageInfo::InstNum => {
                    inst_num = Some(map.next_value()?);
                }
                FieldImageInfo::SopUid => {
                    sop_uid = Some(map.next_value()?);
                }
                FieldImageInfo::Rows => {
                    rows = Some(map.next_value()?);
                }
                FieldImageInfo::Cols => {
                    cols = Some(map.next_value()?);
                }
                FieldImageInfo::PixelSpacing => {
                    pixel_spacing = Some(map.next_value()?);
                }
                FieldImageInfo::SliceLocation => {
                    slice_location = Some(map.next_value()?);
                }
                FieldImageInfo::SliceThickness => {
                    slice_thickness = Some(map.next_value()?);
                }
                FieldImageInfo::ImagePosition => {
                    image_position = Some(map.next_value()?);
                }
                FieldImageInfo::ImageOrientation => {
                    image_orientation = Some(map.next_value()?);
                }
                FieldImageInfo::WindowCenter => {
                    window_center = Some(map.next_value()?);
                }
                FieldImageInfo::WindowWidth => {
                    window_width = Some(map.next_value()?);
                }
                FieldImageInfo::PhotometricInterpretation => {
                    photometric_interpretation = Some(map.next_value()?);
                }
                FieldImageInfo::BitsAllocated => {
                    bits_allocated = Some(map.next_value()?);
                }
                FieldImageInfo::HighBit => {
                    high_bit = Some(map.next_value()?);
                }
                FieldImageInfo::BitStored => {
                    bit_stored = Some(map.next_value()?);
                }
                FieldImageInfo::SamplesPerPixel => {
                    samples_per_pixel = Some(map.next_value()?);
                }
                FieldImageInfo::PixelRepresentation => {
                    pixel_representation = Some(map.next_value()?);
                }
                FieldImageInfo::Ignore => {
                    let _: de::IgnoredAny = map.next_value()?;
                }
            }
        }

        Ok(ImageInfo {
            inst_num: inst_num.ok_or_else(|| de::Error::missing_field("x00200013"))?,
            sop_uid: sop_uid.ok_or_else(|| de::Error::missing_field("x00080018"))?,
            rows: rows.ok_or_else(|| de::Error::missing_field("x00280010"))?,
            cols: cols.ok_or_else(|| de::Error::missing_field("x00280011"))?,
            pixel_spacing: pixel_spacing
                .ok_or_else(|| de::Error::missing_field("x00280030"))?,
            slice_location: slice_location
                .ok_or_else(|| de::Error::missing_field("x00201041"))?,
            slice_thickness: slice_thickness
                .ok_or_else(|| de::Error::missing_field("x00180050"))?,
            image_position: image_position
                .ok_or_else(|| de::Error::missing_field("x00200032"))?,
            image_orientation: image_orientation
                .ok_or_else(|| de::Error::missing_field("x00200037"))?,
            window_center: window_center
                .ok_or_else(|| de::Error::missing_field("x00281050"))?,
            window_width: window_width
                .ok_or_else(|| de::Error::missing_field("x00281051"))?,
            photometric_interpretation: photometric_interpretation
                .ok_or_else(|| de::Error::missing_field("x00280004"))?,
            bits_allocated: bits_allocated
                .ok_or_else(|| de::Error::missing_field("x00280100"))?,
            high_bit: high_bit
                .ok_or_else(|| de::Error::missing_field("x00280101"))?,
            bit_stored: bit_stored
                .ok_or_else(|| de::Error::missing_field("x00280102"))?,
            samples_per_pixel: samples_per_pixel
                .ok_or_else(|| de::Error::missing_field("x00280002"))?,
            pixel_representation: pixel_representation
                .ok_or_else(|| de::Error::missing_field("x00280103"))?,
        })
    }
}

impl<'de> de::Deserialize<'de> for ImageInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &'static [&'static str] = &[
            "x00200013", // INSTANCE_NUMBER
            "x00080018", // SOP_INSTANCE_UID
            "x00280010", // ROWS
            "x00280011", // COLUMNS
            "x00280030", // PIXEL_SPACING
            "x00201041", // SLICE_LOCATION
            "x00180050", // SLICE_THICKNESS
            "x00200032", // IMAGE_POSITION_PATIENT
            "x00200037", // IMAGE_ORIENTATION_PATIENT
            "x00281050", // WINDOW_CENTER
            "x00281051", // WINDOW_WIDTH
            "x00280004", // PHOTOMETRIC_INTERPRETATION
            "x00280100", // BITS_ALLOCATED
            "x00280101", // HIGH_BIT
            "x00280102", // BITS_STORED
            "x00280002", // SAMPLES_PER_PIXEL
            "x00280103", // PIXEL_REPRESENTATION
        ];

        deserializer.deserialize_struct("ImageInfo", FIELDS, ImageInfoVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn dummy_image_info() -> ImageInfo {
        ImageInfo {
            inst_num: 123,
            sop_uid: "1.2.840.113619.2.1.20230101.123456.123".to_string(),
            rows: 512,
            cols: 512,
            pixel_spacing: "0.5\\0.5".to_string(),
            slice_location: "-100.0".to_string(),
            slice_thickness: "5.0".to_string(),
            image_position: "10.0\\20.0\\30.0".to_string(),
            image_orientation: "1\\0\\0\\0\\1\\0".to_string(),
            window_center: "40".to_string(),
            window_width: "400".to_string(),
            photometric_interpretation: "MONOCHROME2".to_string(),
            bits_allocated: 16,
            high_bit: 15,
            bit_stored: 15,
            samples_per_pixel: 1,
            pixel_representation: 0,
        }
    }

    #[test]
    fn test_serialize_image_info() {
        let image = dummy_image_info();
        let json = serde_json::to_string(&image).unwrap();

        let expected_json = r#"
{
  "x00200013":123,
  "x00080018":"1.2.840.113619.2.1.20230101.123456.123",
  "x00280010":512,
  "x00280011":512,
  "x00280030":"0.5\\0.5",
  "x00201041":"-100.0",
  "x00180050":"5.0",
  "x00200032":"10.0\\20.0\\30.0",
  "x00200037":"1\\0\\0\\0\\1\\0",
  "x00281050":"40",
  "x00281051":"400",
  "x00280004":"MONOCHROME2",
  "x00280100":16,
  "x00280101":15,
  "x00280102":15,
  "x00280002":1,
  "x00280103":0
}"#
            .replace("\n", "")
            .replace(" ", "");

        let actual_json = json.replace(" ", "");
        assert_eq!(actual_json, expected_json);
    }

    #[test]
    fn test_deserialize_image_info() {
        let json = r#"
{
  "x00200013":123,
  "x00080018":"1.2.840.113619.2.1.20230101.123456.123",
  "x00280010":512,
  "x00280011":512,
  "x00280030":"0.5\\0.5",
  "x00201041":"-100.0",
  "x00180050":"5.0",
  "x00200032":"10.0\\20.0\\30.0",
  "x00200037":"1\\0\\0\\0\\1\\0",
  "x00281050":"40",
  "x00281051":"400",
  "x00280004":"MONOCHROME2",
  "x00280100":16,
  "x00280101":15,
  "x00280102":15,
  "x00280002":1,
  "x00280103":0
}"#;

        let image: ImageInfo = serde_json::from_str(json).unwrap();

        assert_eq!(image.inst_num, 123);
        assert_eq!(
            image.sop_uid,
            "1.2.840.113619.2.1.20230101.123456.123"
        );
        assert_eq!(image.rows, 512);
        assert_eq!(image.cols, 512);
        assert_eq!(image.pixel_spacing, "0.5\\0.5");
        assert_eq!(image.slice_location, "-100.0");
        assert_eq!(image.slice_thickness, "5.0");
        assert_eq!(image.image_position, "10.0\\20.0\\30.0");
        assert_eq!(image.image_orientation, "1\\0\\0\\0\\1\\0");
        assert_eq!(image.window_center, "40");
        assert_eq!(image.window_width, "400");
        assert_eq!(image.photometric_interpretation, "MONOCHROME2");
        assert_eq!(image.bits_allocated, 16);
        assert_eq!(image.high_bit, 15);
        assert_eq!(image.bit_stored, 15);
        assert_eq!(image.samples_per_pixel, 1);
        assert_eq!(image.pixel_representation, 0);
    }
}
