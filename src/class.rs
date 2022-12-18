use crate::class_file::ClassFile;
use crate::constant_pool::ConstantPoolInfo;
use crate::field_info::Field;
use crate::method::{JavaMethod, Method};
use crate::{ClassReader, Runtime};
use std::collections::HashMap;
use crate::things::Value;

pub enum Class {
    Java(JavaClass),
    Native(NativeClass),
}

pub struct NativeClass {
    pub name: String,
    pub access_flags: u16,
    pub super_class: *mut NativeClass,
    pub interfaces: Vec<*mut NativeClass>,
    pub static_fields: HashMap<(String, String), Field>,
}

#[derive(Debug)]
pub struct JavaClass {
    pub name: String,
    pub constant_pool: Vec<ConstantPoolInfo>,
    pub access_flags: u16,
    pub super_class: *mut Class,
    pub interfaces: Vec<*mut Class>, // sorted
    pub static_fields: HashMap<(String, String), Field>,
    pub instance_fields: HashMap<(String, String), Field>,
    pub methods: HashMap<(String, String), Method>, // (Name, Descriptor)
    pub attributes: HashMap<String, Vec<u8>>,           // String is name, Vec is data
}

impl<'a> JavaClass {
    pub fn from_filename(name: &str, runtime: &mut Runtime) -> Result<JavaClass, std::io::Error> {
        return Ok(Self::from_classfile(
            ClassReader::new(name)?.read_classfile(),
            runtime
        ));
    }

    pub fn from_classfile(mut c: ClassFile, runtime: &mut Runtime) -> JavaClass {
        let cp = &mut c.constant_pool;

        let mut cls = JavaClass {
            access_flags: c.access_flags,
            name: cp[(c.this_class - 1) as usize].class_name(cp),
            super_class: runtime.load(cp[(c.super_class - 1) as usize].class_name(cp))
                .expect("loading super class failed!"),
            interfaces: c
                .interfaces
                .iter()
                .map(|x| &mut JavaClass::from_filename(
                    &*cp[(x - 1) as usize].class_name(cp),
                    runtime
                ).unwrap() as *mut JavaClass).collect(),
            static_fields: HashMap::new(),
            instance_fields: HashMap::new(),
            methods: HashMap::new(),
            attributes: HashMap::new(),
            constant_pool: cp.clone(),
        };

        for fi in &c.fields {
            let f = Field::from_info(cp, fi);
            if f.is_static() {
                cls.static_fields.insert((f.name.clone(), f.descriptor.clone()), f);
            } else {
                cls.field_order.insert(
                    cls.field_order.binary_search(&f.name)
                        .unwrap_err(),
                    f.name.clone()
                );
            }
        }

        for mi in &c.methods {
            let m = JavaMethod::from_info(cp, mi);
            cls.methods
                .insert((m.name.clone(), m.descriptor.clone()), Method::Java(m));
        }
        for a in &c.attributes {
            cls.attributes.insert(
                cp[(a.name_index - 1) as usize]
                    .utf8()
                    .expect("attribute name pointer to invalid index"),
                a.clone().info,
            );
        }
        cls
    }

    pub fn get_method(&'a self, name: String, descriptor: String) -> Result<&'a Method, ()> {
        if let Some(m) = self.methods.get(&(name, descriptor)) {
            return Ok(m);
        }
        return Err(());
    }

    pub fn set_static(&mut self, field: &(String, String), val: Value) -> Result<(), ()> {
        Ok(self.static_fields.get_mut(field).ok_or(())?.set_static(val))
    }

    pub fn get_static(&self, field: &(String, String)) -> Option<Value> {
        Some(self.static_fields.get(field)?.get_static())
    }

    pub fn get_instance_field(&'a self, field: &(String, String)) -> Result<&'a Field, ()> {
        self.instance_fields.get(field).ok_or(())
    }
}
