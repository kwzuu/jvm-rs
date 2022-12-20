use std::collections::hash_map::Iter;
use crate::class_file::ClassFile;
use crate::constant_pool::ConstantPoolInfo;
use crate::field_info::Field;
use crate::method::{JavaMethod, Method};
use crate::{ClassReader, Runtime};
use std::collections::HashMap;
use crate::class::Class::{Java, Native};
use crate::things::Value;

pub(crate) mod access_flags {
    pub const PUBLIC: u16 = 0x0001;
    pub const FINAL: u16 = 0x0010;
    pub const SUPER: u16 = 0x0020;
    pub const INTERFACE: u16 = 0x0200;
    pub const ABSTRACT: u16 = 0x0400;
    pub const SYNTHETIC: u16 = 0x1000;
    pub const ANNOTATION: u16 = 0x2000;
    pub const ENUM: u16 = 0x4000;
}
#[derive(Debug)]
pub enum Class {
    Java(JavaClass),
    Native(NativeClass),
}

impl Class {
    pub fn instance_fields(&self) -> Iter<'_, String, Field> {
        match self {
            Java(c) => c.instance_fields.iter(),
            Native(c) => c.instance_fields.iter(),
        }
    }

    pub fn interfaces(&self) -> &Vec<*mut Class> {
        match self {
            Java(c) => &c.interfaces,
            Native(c) => &c.interfaces,
        }
    }

    pub fn super_class(&self) -> *mut Class {
        match self {
            Java(c) => c.super_class,
            Native(c) => c.super_class
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Java(c) => &*c.name,
            Native(c) => &*c.name,
        }
    }
    pub fn static_fields(&self) -> &HashMap<String, Field> {
        match self {
            Java(c) => &c.static_fields,
            Native(c) => &c.static_fields,
        }
    }
    pub fn instance_fields_raw(&self) -> &HashMap<String, Field> {
        match self {
            Java(c) => &c.instance_fields,
            Native(c) => &c.instance_fields,
        }
    }
    pub fn methods(&self) -> &HashMap<(String, String), Method> {
        match self {
            Java(c) => &c.methods,
            Native(c) => &c.methods,
        }
    }

    pub fn access_flags(&self) -> u16 {
        match self {
            Java(c) => c.access_flags,
            Native(c) => c.access_flags,
        }
    }

    pub fn java(&self) -> Option<*const JavaClass> {
        match self {
            Java(c) => Some(c as *const JavaClass),
            Native(_) => None
        }
    }

    pub fn java_mut(&mut self) -> Option<*mut JavaClass> {
        match self {
            Java(c) => Some(c as *mut JavaClass),
            Native(_) => None
        }
    }

    pub fn native(&mut self) -> Option<*const NativeClass> {
        match self {
            Java(_) => None,
            Native(c) => Some(c as *const NativeClass)
        }
    }
    pub fn native_mut(&mut self) -> Option<*mut NativeClass> {
        match self {
            Java(_) => None,
            Native(c) => Some(c as *mut NativeClass)
        }
    }

    pub fn get_method(&self, name: String, descriptor: String) -> Option<&Method> {
        match self {
            Java(c) => c.methods.get(&(name, descriptor)),
            Native(c) => c.methods.get(&(name, descriptor)),
        }
    }

    pub fn get_static(&self, name: &str) -> Option<Value> {
        match self {
            Java(c) => &c.static_fields,
            Native(c) => &c.static_fields,
        }.get(name).map(Field::get_static)
    }
}
#[derive(Debug)]
pub struct NativeClass {
    pub name: String,
    pub access_flags: u16,
    pub super_class: *mut Class,
    pub interfaces: Vec<*mut Class>,
    pub static_fields: HashMap<String, Field>,
    pub instance_fields: HashMap<String, Field>,
    pub methods: HashMap<(String, String), Method>,
}

#[derive(Debug)]
pub struct JavaClass {
    pub name: String,
    pub constant_pool: Vec<ConstantPoolInfo>,
    pub access_flags: u16,
    pub super_class: *mut Class,
    pub interfaces: Vec<*mut Class>, // sorted
    pub static_fields: HashMap<String, Field>,
    pub instance_fields: HashMap<String, Field>,
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
            interfaces: c.interfaces.iter()
                .map(|x|
                    runtime.load(
                        cp[(x - 1) as usize].utf8().unwrap()
                    ).unwrap()
                ).collect(),
            static_fields: HashMap::new(),
            instance_fields: HashMap::new(),
            methods: HashMap::new(),
            attributes: HashMap::new(),
            constant_pool: cp.clone(),
        };

        for fi in &c.fields {
            let f = Field::from_info(cp, fi);
            if f.is_static() {
                &mut cls.static_fields
            } else {
                &mut cls.instance_fields
            }.insert(f.name.clone(), f);
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

    pub fn set_static(&mut self, field: &str, val: Value) -> Result<(), ()> {
        Ok(self.static_fields.get_mut(field).ok_or(())?.set_static(val))
    }

    pub fn get_static(&self, field: &str) -> Option<Value> {
        Some(self.static_fields.get(field)?.get_static())
    }

    pub fn get_instance_field(&'a self, field: &str) -> Result<&'a Field, ()> {
        self.instance_fields.get(field).ok_or(())
    }
}
