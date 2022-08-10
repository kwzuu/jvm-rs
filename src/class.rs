use crate::class_file::ClassFile;
use crate::constant_pool::ConstantPoolInfo;
use crate::field_info::Field;
use crate::method::Method;
use crate::ClassReader;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Class {
    pub version: (u16, u16), // (major, minor)
    pub constant_pool: Vec<ConstantPoolInfo>,
    pub access_flags: u16,
    pub this_class: String,
    pub super_class: String,
    pub interfaces: Vec<String>, // sorted
    pub fields: HashMap<String, Rc<Field>>,
    pub methods: HashMap<(String, String), Rc<Method>>, // (Name, Descriptor)
    pub attributes: HashMap<String, Vec<u8>>,           // String is name, Vec is data
}

impl<'a> Class {
    pub fn from_filename(name: &String) -> Result<Class, std::io::Error> {
        return Ok(Self::from_classfile(
            ClassReader::new(name)?.read_classfile(),
        ));
    }

    pub(crate) fn from_classfile(c: ClassFile) -> Class {
        let mut new_cp = vec![];
        c.constant_pool.clone_into(&mut new_cp);
        let new_cp = &mut new_cp;

        let mut cls = Class {
            version: (c.major_version, c.minor_version),
            constant_pool: new_cp.clone(),
            access_flags: c.access_flags,
            this_class: c.constant_pool[(c.this_class - 1) as usize].class_name(new_cp),
            super_class: c.constant_pool[(c.super_class - 1) as usize].class_name(new_cp),
            interfaces: c
                .interfaces
                .iter()
                .map(|x| c.constant_pool[(x - 1) as usize].class_name(new_cp))
                .collect(),
            fields: HashMap::new(),
            methods: HashMap::new(),
            attributes: HashMap::new(),
        };
        for fi in &c.fields {
            let f = Field::from_info(new_cp, fi);
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

    pub fn get_method(&self, name: &'a String, descriptor: &'a String) -> Result<Rc<Method>, ()> {
        if let Some(m) = self.methods.get(&(name.clone(), descriptor.clone())) {
            return Ok(m.clone());
        }
        return Err(());
    }

    pub fn get_field(&self, name: &'a String) -> Result<Rc<Field>, ()> {
        match self.fields.get(name) {
            Some(f) => Ok(f.clone()),
            _ => Err(()),
        }
    }
}
