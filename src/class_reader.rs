use std::fs::File;
use std::io::Read;
use crate::attribute_info::AttributeInfo;
use crate::class_file::ClassFile;
use crate::constant_pool::ConstantPoolInfo;
use crate::constant_pool::representations::*;
use crate::field_info::FieldInfo;
use crate::method_info::MethodInfo;

pub struct ClassReader {
    bytes: std::io::Bytes<File>,
}

impl ClassReader {
    pub(crate) fn new(path: String) -> ClassReader {
        let f: File = File::open(path).expect("open fail");
        ClassReader {
            bytes: f.bytes(),
        }
    }

    fn read_n<'a>(&mut self, n: usize) -> Vec<u8> {
        let mut b = Vec::with_capacity(n);
        for _ in 0..n {
            b.push(self.read_u1())
        }
        b
    }

    fn read_u1(&mut self) -> u8 {
        self.bytes.next().unwrap().expect("read fail")
    }

    fn read_u2(&mut self) -> u16 {
        let hi = self.read_u1() as u16;
        let lo = self.read_u1() as u16;
        hi << 8 | lo
    }

    fn read_u4(&mut self) -> u32 {
        let hi = self.read_u2() as u32;
        let lo = self.read_u2() as u32;
        hi << 16 | lo
    }

    fn read_cpinfo(&mut self) -> ConstantPoolInfo {
        match self.read_u1() {
            7 => ConstantPoolInfo::Class(Class {
                name_index: self.read_u2()
            }),
            9 => ConstantPoolInfo::Fieldref(Fieldref {
                class_index: self.read_u2(),
                name_and_type_index: self.read_u2()
            }),
            10 => ConstantPoolInfo::Methodref(Methodref {
                class_index: self.read_u2(),
                name_and_type_index: self.read_u2()
            }),
            11 => ConstantPoolInfo::InterfaceMethodref(InterfaceMethodref {
                class_index: self.read_u2(),
                name_and_type_index: self.read_u2()
            }),
            8 => ConstantPoolInfo::JString(JString {
                string_index: self.read_u2()
            }),
            3 => ConstantPoolInfo::Integer(Integer {
                bytes: self.read_u4()
            }),
            4 => ConstantPoolInfo::Float(Float {
                bytes: self.read_u4()
            }),
            5 => ConstantPoolInfo::Long(Long {
                high_bytes: self.read_u4(),
                low_bytes: self.read_u4()
            }),
            6 => ConstantPoolInfo::Double(Double {
                high_bytes: self.read_u4(),
                low_bytes: self.read_u4()
            }),
            12 => ConstantPoolInfo::NameAndType( NameAndType {
                name_index: self.read_u2(),
                descriptor_index: self.read_u2()
            }),
            1 => {
                let l = self.read_u2();
                ConstantPoolInfo::Utf8(Utf8 {
                    length: l,
                    bytes: self.read_n(l as usize)
                })
            },
            15 => ConstantPoolInfo::MethodHandle(MethodHandle {
                reference_kind: self.read_u1(),
                reference_index: self.read_u2()
            }),
            16 => ConstantPoolInfo::MethodType(MethodType {
                descriptor_index: self.read_u2()
            }),
            18 => ConstantPoolInfo::InvokeDynamic(InvokeDynamic {
                bootstrap_method_attr_index: self.read_u2(),
                name_and_type_index: self.read_u2()
            }),
            x => panic!("invalid constant pool tag {}", x)
        }
    }

    fn read_fieldinfo(&mut self) -> FieldInfo {
        let mut f = FieldInfo {
            access_flags: self.read_u2(),
            name_index: self.read_u2(),
            descriptor_index: self.read_u2(),
            attributes_count: self.read_u2(),
            attributes: vec![]
        };

        for _ in 0..f.attributes_count {
            f.attributes.push(self.read_attributeinfo())
        }

        f
    }

    fn read_methodinfo(&mut self) -> MethodInfo {
        let mut m = MethodInfo {
            access_flags: self.read_u2(),
            name_index: self.read_u2(),
            descriptor_index: self.read_u2(),
            attributes_count: self.read_u2(),
            attributes: vec![]
        };

        for _ in 0..m.attributes_count {
            m.attributes.push(self.read_attributeinfo())
        }

        m
    }

    fn read_attributeinfo(&mut self) -> AttributeInfo {
        let mut a = AttributeInfo {
            name_index: self.read_u2(),
            attribute_length: self.read_u4(),
            info: vec![],
        };

        for _ in 0..a.attribute_length {
            a.info.push(self.read_u1())
        }

        a
    }

    pub(crate) fn read_class(&mut self) -> ClassFile {
        let mut cf = ClassFile {
            magic: self.read_u4(),
            minor_version: self.read_u2(),
            major_version: self.read_u2(),
            constant_pool_count: self.read_u2(),
            constant_pool: vec![],
            access_flags: 0,
            this_class: 0,
            super_class: 0,
            interfaces_count: 0,
            interfaces: vec![],
            fields_count: 0,
            fields: vec![],
            methods_count: 0,
            methods: vec![],
            attributes_count: 0,
            attributes: vec![]
        };

        for _ in 0..(cf.constant_pool_count - 1) {
            let c = self.read_cpinfo();
            cf.constant_pool.push(c)
        }
        cf.access_flags = self.read_u2();
        cf.this_class = self.read_u2();
        cf.super_class = self.read_u2();
        cf.interfaces_count = self.read_u2();
        for _ in 0..cf.interfaces_count {
            cf.interfaces.push(self.read_u2())
        }
        cf.fields_count = self.read_u2();
        for _ in 0..cf.fields_count {
            cf.fields.push(self.read_fieldinfo())
        }
        cf.methods_count = self.read_u2();
        for _ in 0..cf.methods_count {
            cf.methods.push(self.read_methodinfo())
        }
        cf.attributes_count = self.read_u2();
        for _ in 0..cf.attributes_count {
            cf.attributes.push(self.read_attributeinfo())
        }
        cf
    }
}