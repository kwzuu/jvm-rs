use std::collections::HashMap;
use crate::attribute_info::AttributeInfo;

#[derive(Debug)]
pub struct FieldInfo {
    pub(crate) access_flags: u16,
    pub(crate) name_index: u16,
    pub(crate) descriptor_index: u16,
    pub(crate) attributes_count: u16,
    pub(crate) attributes: Vec<AttributeInfo>,
}

pub struct Field {
    pub access_flags: u16,
    pub name: String,
    pub descriptor: String,
    pub attributes: HashMap<String, Vec<u8>>
}