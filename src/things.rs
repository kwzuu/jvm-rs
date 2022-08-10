use crate::Class;
use std::rc::Rc;

pub struct Object {
    class: Rc<Class>,
    fields: Rc<[Thing]>,
}

pub enum Thing {
    Byte(i8),
    Char(char),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Object(Rc<Object>),
    BArray(Rc<[i8]>),
    CArray(Rc<[char]>),
    SArray(Rc<[i16]>),
    IArray(Rc<[i32]>),
    LArray(Rc<[i64]>),
    FArray(Rc<[f32]>),
    DArray(Rc<[f64]>),
    OArray(Rc<[Rc<Object>]>),
}

impl Thing {
    pub fn byte(self) -> i8 {
        if let Thing::Byte(x) = self {
            return x;
        }
        panic!("TYPE ERROR")
    }

    pub fn char(self) -> char {
        if let Thing::Char(x) = self {
            return x;
        }
        panic!("TYPE ERROR")
    }

    pub fn int(self) -> i32 {
        if let Thing::Int(x) = self {
            return x;
        }
        panic!("TYPE ERROR")
    }

    pub fn long(self) -> i64 {
        if let Thing::Long(x) = self {
            return x;
        }
        panic!("TYPE ERROR")
    }

    pub fn float(self) -> f32 {
        if let Thing::Float(x) = self {
            return x;
        }
        panic!("TYPE ERROR")
    }

    pub fn double(self) -> f64 {
        if let Thing::Double(x) = self {
            return x;
        }
        panic!("TYPE ERROR")
    }

    pub fn object(self) -> Rc<Object> {
        if let Thing::Object(x) = self {
            return x;
        }
        panic!("TYPE ERROR")
    }

    pub fn abyte(self) -> Rc<[i8]> {
        if let Thing::BArray(x) = self {
            return x;
        }
        panic!("TYPE ERROR")
    }

    pub fn ashort(self) -> Rc<[i16]> {
        if let Thing::SArray(x) = self {
            return x;
        }
        panic!("TYPE ERROR")
    }

    pub fn achar(self) -> Rc<[char]> {
        if let Thing::CArray(x) = self {
            return x;
        }
        panic!("TYPE ERROR")
    }

    pub fn aint(self) -> Rc<[i32]> {
        if let Thing::IArray(x) = self {
            return x;
        }
        panic!("TYPE ERROR")
    }

    pub fn along(self) -> Rc<[i64]> {
        if let Thing::LArray(x) = self {
            return x;
        }
        panic!("TYPE ERROR")
    }

    pub fn afloat(self) -> Rc<[f32]> {
        if let Thing::FArray(x) = self {
            return x;
        }
        panic!("TYPE ERROR")
    }

    pub fn adouble(self) -> Rc<[f64]> {
        if let Thing::DArray(x) = self {
            return x;
        }
        panic!("TYPE ERROR")
    }

    pub fn aobject(self) -> Rc<[Rc<Object>]> {
        if let Thing::OArray(x) = self {
            return x;
        }
        panic!("TYPE ERROR")
    }
}
