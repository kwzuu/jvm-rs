use std::collections::HashMap;
use crate::attribute_info::AttributeInfo;
use crate::constant_pool::ConstantPoolInfo;
use crate::field_info::{Field, FieldInfo};
use crate::method_info::{Method, MethodInfo};

#[derive(Debug)]
pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
    pub constant_pool: Vec<ConstantPoolInfo>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Vec<u16>,
    pub fields_count: u16,
    pub fields: Vec<FieldInfo>,
    pub methods_count: u16,
    pub methods: Vec<MethodInfo>,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
pub struct Class {
    pub version: (u16, u16), // (major, minor)
    pub constant_pool: Vec<ConstantPoolInfo>,
    pub access_flags: u16,
    pub this_class: String,
    pub super_class: String,
    pub interfaces: Vec<String>, // sorted
    pub fields: Vec<Field>,
    pub methods: HashMap<String, Method>,
    pub attributes: HashMap<String, Vec<u8>>, // String is name, Vec is data
}

impl Class {
    pub(crate) fn new(c: &ClassFile) -> Class {
        let mut new_cp = vec![];
        c.constant_pool.clone_into(&mut new_cp);
        let mut new_cp = &mut new_cp;
        let mut cls = Class {
            version: (c.major_version, c.minor_version),
            constant_pool: new_cp.clone(),
            access_flags: c.access_flags,
            this_class: c.constant_pool[(c.this_class - 1) as usize].class_name(new_cp),
            super_class: c.constant_pool[(c.super_class - 1) as usize].class_name(new_cp),
            interfaces: c.interfaces.iter()
                .map(|x| c.constant_pool[(x - 1) as usize].class_name(new_cp))
                .collect(),
            fields: vec![],
            methods: HashMap::new(),
            attributes: HashMap::new()
        };
        for m in &c.methods {
            cls.methods.insert(
                c.constant_pool[(m.name_index - 1) as usize]
                    .utf8().expect("method name pointer to invalid index"),
                Method::from_info(&cls.constant_pool, m)
            );
        }
        for a in &c.attributes {
            cls.attributes.insert(
                c.constant_pool[(a.name_index - 1) as usize]
                    .utf8().expect("attribute name pointer to invalid index"),
                a.clone().info
            );
        }
        cls
    }
}
