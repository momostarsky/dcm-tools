 
use dicom::core::Tag;
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

pub fn get_string(tag: Tag, obj: &DefaultDicomObject) -> String {
    get_tag_value(tag, obj, "".to_string())
}
// pub fn tag_name(tag: Tag) -> String {
//     format!("{:?}", tag)
// }
// pub fn json_key(tag: Tag) -> String {
//     let group = tag.group();
//     let elem = tag.element();
//     format!("x{:04X}{:04X}", group, elem)
// }
