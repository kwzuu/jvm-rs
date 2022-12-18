use representations::*;

#[derive(Debug, Clone)]
pub enum ConstantPoolInfo {
    Class(Class),
    Fieldref(Fieldref),
    Methodref(Methodref),
    InterfaceMethodref(InterfaceMethodref),
    JString(JString),
    Integer(Integer),
    Float(Float),
    Long(Long),
    Double(Double),
    NameAndType(NameAndType),
    Utf8(Utf8),
    MethodHandle(MethodHandle),
    MethodType(MethodType),
    InvokeDynamic(InvokeDynamic),
}

pub mod representations {
    use std::fmt::{Debug, Formatter, Write};

    #[derive(Debug, Clone)]
    pub struct Class {
        pub name_index: u16,
    }
    #[derive(Debug, Clone)]
    pub struct Fieldref {
        pub class_index: u16,
        pub name_and_type_index: u16,
    }
    #[derive(Debug, Clone)]
    pub struct Methodref {
        pub class_index: u16,
        pub name_and_type_index: u16,
    }
    #[derive(Debug, Clone)]
    pub struct InterfaceMethodref {
        pub class_index: u16,
        pub name_and_type_index: u16,
    }
    #[derive(Debug, Clone)]
    pub struct JString {
        pub string_index: u16,
    }
    #[derive(Debug, Clone)]
    pub struct Integer {
        pub bytes: u32,
    }
    #[derive(Debug, Clone)]
    pub struct Float {
        pub bytes: u32,
    }
    #[derive(Debug, Clone)]
    pub struct Long {
        pub high_bytes: u32,
        pub low_bytes: u32,
    }
    #[derive(Debug, Clone)]
    pub struct Double {
        pub high_bytes: u32,
        pub low_bytes: u32,
    }
    #[derive(Debug, Clone)]
    pub struct NameAndType {
        pub name_index: u16,
        pub descriptor_index: u16,
    }

    pub struct Utf8 {
        pub length: u16,
        pub bytes: Vec<u8>,
    }
    #[derive(Debug, Clone)]
    pub struct MethodHandle {
        pub reference_kind: u8,
        pub reference_index: u16,
    }
    #[derive(Debug, Clone)]
    pub struct MethodType {
        pub descriptor_index: u16,
    }
    #[derive(Debug, Clone)]
    pub struct InvokeDynamic {
        pub bootstrap_method_attr_index: u16,
        pub name_and_type_index: u16,
    }

    impl Debug for Utf8 {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_char('"')?;
            f.write_str(std::str::from_utf8(self.bytes.as_slice()).expect("bad utf8"))?;
            f.write_char('"')?;
            Ok(())
        }
    }

    impl Clone for Utf8 {
        fn clone(&self) -> Self {
            let mut bytes = Vec::with_capacity(self.bytes.len());
            self.bytes.clone_into(&mut bytes);
            Utf8 {
                length: self.length,
                bytes,
            }
        }
    }
}

impl ConstantPoolInfo {
    pub fn utf8(&self) -> Option<String> {
        if let ConstantPoolInfo::Utf8(u) = self {
            String::from_utf8(Vec::from(u.bytes.as_slice())).ok()
        } else {
            None
        }
    }

    pub fn class_name(&self, cp: &Vec<ConstantPoolInfo>) -> String {
        cp[(self.class().unwrap().name_index - 1) as usize]
            .utf8()
            .unwrap()
    }

    pub fn class(&self) -> Option<Class> {
        if let ConstantPoolInfo::Class(c) = self {
            Some(c.clone())
        } else {
            None
        }
    }

    pub fn fieldref(&self) -> Option<Fieldref> {
        if let ConstantPoolInfo::Fieldref(fr) = self {
            Some(fr.clone())
        } else {
            None
        }
    }

    pub fn method_type(&self) -> Option<MethodType> {
        if let ConstantPoolInfo::MethodType(mt) = self {
            Some(mt.clone())
        } else {
            None
        }
    }

    pub fn method_handle(&self) -> Option<MethodHandle> {
        if let ConstantPoolInfo::MethodHandle(mh) = self {
            Some(mh.clone())
        } else {
            None
        }
    }

    pub fn methodref(&self) -> Option<Methodref> {
        if let ConstantPoolInfo::Methodref(mr) = self {
            Some(mr.clone())
        } else {
            None
        }
    }

    pub fn name_and_type(&self) -> Option<NameAndType> {
        if let ConstantPoolInfo::NameAndType(nt) = self {
            Some(nt.clone())
        } else {
            None
        }
    }

    pub fn tag(self) -> u8 {
        use ConstantPoolInfo::*;

        match self {
            Class(_) => 7,
            Fieldref(_) => 9,
            Methodref(_) => 10,
            InterfaceMethodref(_) => 11,
            JString(_) => 8,
            Integer(_) => 3,
            Float(_) => 4,
            Long(_) => 5,
            Double(_) => 6,
            NameAndType(_) => 12,
            Utf8(_) => 1,
            MethodHandle(_) => 15,
            MethodType(_) => 16,
            InvokeDynamic(_) => 18,
        }
    }
}
