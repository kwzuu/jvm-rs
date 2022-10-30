use crate::attributes::attribute_info::AttributeInfo;
use crate::constant_pool::ConstantPoolInfo;
use std::collections::HashMap;

#[derive(Debug)]
pub struct FieldInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub access_flags: u16,
    pub name: String,
    pub descriptor: String,
    pub attributes: HashMap<String, Vec<u8>>,
}

impl Field {
    pub(crate) fn from_info(cp: &Vec<ConstantPoolInfo>, fi: &FieldInfo) -> Field {
        let mut f = Field {
            access_flags: fi.access_flags,
            name: cp[(fi.name_index - 1) as usize]
                .utf8()
                .expect("bad utf8 in field name"),
            descriptor: cp[(fi.descriptor_index - 1) as usize]
                .utf8()
                .expect("bad utf8 in field descriptor"),
            attributes: HashMap::new(),
        };
        for a in &fi.attributes {
            f.attributes.insert(
                cp[(a.name_index - 1) as usize]
                    .utf8()
                    .expect("bad utf8 in attribute name"),
                a.info.clone(),
            );
        }
        f
    }
}
