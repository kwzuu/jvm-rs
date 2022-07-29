use std::collections::HashMap;
use crate::attribute_info::AttributeInfo;
use crate::constant_pool::ConstantPoolInfo;

#[derive(Debug)]
pub struct MethodInfo {
    pub(crate) access_flags: u16,
    pub(crate) name_index: u16,
    pub(crate) descriptor_index: u16,
    pub(crate) attributes_count: u16,
    pub(crate) attributes: Vec<AttributeInfo>
}

impl Clone for MethodInfo {
    fn clone(&self) -> Self {
        let mut attrs = vec![];
        self.attributes.clone_into(&mut attrs);
        MethodInfo {
            access_flags: self.access_flags,
            name_index: self.name_index,
            descriptor_index: self.descriptor_index,
            attributes_count: self.attributes_count,
            attributes: attrs,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Method {
    pub access_flags: u16,
    pub descriptor: String,
    pub attributes: HashMap<String, Vec<u8>>
}

impl Method {
    pub fn from_info(cp: &Vec<ConstantPoolInfo>, m: &MethodInfo) -> Method {
        let ret = Method {
            access_flags: m.access_flags,
            attributes: HashMap::new(),
            descriptor: cp[(m.descriptor_index - 1) as usize]
                .utf8().expect("bad utf8 for referenced descriptor")
        };
        ret
    }
}