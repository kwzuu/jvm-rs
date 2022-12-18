use crate::attributes::attribute_info::AttributeInfo;
use crate::constant_pool::ConstantPoolInfo;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::things::{Object, Value};


#[derive(Debug)]
pub struct FieldInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Copy, Clone)]
pub union AccessHelper {
    offset: usize,
    pub(crate) value: Value,
}

impl Debug for AccessHelper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Value as Debug>::fmt(&unsafe { self.value }, f)
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub access_flags: u16,
    pub name: String,
    pub descriptor: String,
    pub attributes: HashMap<String, Vec<u8>>,
    pub(crate) access_helper: AccessHelper,
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
            access_helper: AccessHelper { offset: 0 }
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

    pub fn get_static(&self) -> Value {
        unsafe { self.access_helper.value }
    }

    pub fn set_static(&mut self, val: Value) {
        self.access_helper = AccessHelper { value: val }
    }

    pub fn get_instance(&self, obj: *const Object) -> Value {
        unsafe {
            (*obj).get(self.access_helper.offset)
        }
    }

    pub fn set_instance(&self, obj: *mut Object, val: Value) {
        unsafe {
            (*obj).set(self.access_helper.offset, val)
        }
    }

    pub fn is_static(&self) -> bool {
        self.access_flags & 0x0008 != 0
    }

    pub fn is_object(&self) -> bool {
        self.descriptor.chars().nth(0).unwrap() == 'L'
    }
}
