use std::borrow::Borrow;
use crate::class_file::ClassFile;
use crate::constant_pool::ConstantPoolInfo;
use crate::field_info::Field;
use crate::method::Method;
use crate::ClassReader;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub constant_pool: Vec<ConstantPoolInfo>,
    pub access_flags: u16,
    pub super_class: String,
    pub interfaces: Vec<Rc<Class>>, // sorted
    pub static_fields: HashMap<String, Field>,
    pub instance_fields: HashMap<String, Field>,
    pub methods: HashMap<(String, String), Rc<Method>>, // (Name, Descriptor)
    pub attributes: HashMap<String, Vec<u8>>,           // String is name, Vec is data
    pub field_order: Vec<String>
}

impl<'a> Class {
    pub fn from_filename(name: &str) -> Result<Class, std::io::Error> {
        return Ok(Self::from_classfile(
            ClassReader::new(name)?.read_classfile(),
        ));
    }

    pub fn from_classfile(c: ClassFile) -> Class {
        let mut new_cp = vec![];
        c.constant_pool.clone_into(&mut new_cp);
        let new_cp = &mut new_cp;

        let mut cls = Class {
            constant_pool: new_cp.clone(),
            access_flags: c.access_flags,
            name: c.constant_pool[(c.this_class - 1) as usize].class_name(new_cp),
            super_class: c.constant_pool[(c.super_class - 1) as usize].class_name(new_cp),
            interfaces: c
                .interfaces
                .iter()
                .map(|x| c.constant_pool[(x - 1) as usize].class_name(new_cp))
                .collect(),
            static_fields: HashMap::new(),
            instance_fields: HashMap::new(),
            methods: HashMap::new(),
            attributes: HashMap::new(),
            field_order: vec![],
        };
        for fi in &c.fields {
            let f = Field::from_info(new_cp, fi);
            cls.field_order.insert(
                cls.field_order.binary_search(&f.name)
                    .err().unwrap(),
                f.name.clone()
            );
            cls.fields.insert(f.name.clone(), Rc::new(f));
        }
        for mi in &c.methods {
            let m = Method::from_info(new_cp, mi);
            cls.methods
                .insert((m.name.clone(), m.descriptor.clone()), Rc::new(m));
        }
        for a in &c.attributes {
            cls.attributes.insert(
                c.constant_pool[(a.name_index - 1) as usize]
                    .utf8()
                    .expect("attribute name pointer to invalid index"),
                a.clone().info,
            );
        }
        cls
    }

    pub fn get_method(&self, name: String, descriptor: String) -> Result<Rc<Method>, ()> {
        if let Some(m) = self.methods.get(&(name, descriptor)) {
            return Ok(m.clone());
        }
        return Err(());
    }

    pub fn get_field(&self, name: &String) -> Result<*const Field, ()> {
        match self.static_fields.get(name).or_else(|| self.instance_fields.get(name)) {
            Some(f) => Ok(f as *const Field as *mut Field),
            _ => Err(()),
        }
    }
}
