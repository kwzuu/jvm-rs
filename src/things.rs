use crate::JavaClass;

use std::fmt::{Display, Debug, Formatter};


#[derive(Copy, Clone)]
pub union Value {
    byte: i8,
    char: u16,
    short: i16,
    int: i32,
    long: i64,
    float: f32,
    double: f64,
    pub(crate) object: *mut Object,
    array: *mut Array,
}

pub struct Object {
    // a pointer to the class the object is from
    pub class: *const JavaClass,
    // we allocate the object with more size than this, because if we made
    // the size of the array variable it would make `Object` `!Sized`, making
    // its pointers 2* fatter and making everything take up twice the memory
    // (very bad for efficiency). this is just a placeholder area, we trick
    // the compiler to keep Object `Sized` and secretly read/write past it.
    pub fields: [Value; 0],
}

impl Object {
    pub fn get(&self, n: usize) -> Value {
        unsafe { self.fields.get_unchecked(n).clone() }
    }

    pub fn set(&mut self, n: usize, val: Value) {
        unsafe {
            self.fields
                .as_mut_ptr()
                .add(n)
                .write(val);

        }
    }
}

pub struct Array {
    ptr: *mut (),
    len: usize,
}

impl Value {
    pub const DCONST_0: Self = Value { double: 0f64 };
    pub const DCONST_1: Self = Value { double: 1f64 };

    pub const FCONST_0: Self = Value { float: 0f32 };
    pub const FCONST_1: Self = Value { float: 1f32 };
    pub const FCONST_2: Self = Value { float: 2f32 };

    pub const ICONST_M1: Self = Value { int: -1 };
    pub const ICONST_0: Self = Value { int: 0 };
    pub const ICONST_1: Self = Value { int: 1 };
    pub const ICONST_2: Self = Value { int: 2 };
    pub const ICONST_3: Self = Value { int: 3 };
    pub const ICONST_4: Self = Value { int: 4 };
    pub const ICONST_5: Self = Value { int: 5 };

    pub const LCONST_0: Self = Value { long: 0 };
    pub const LCONST_1: Self = Value { long: 1 };

    pub fn nbyte(n: i8) -> Self { Self { byte: n } }
    pub fn nchar(n: u16) -> Self { Self { char: n } }
    pub fn nshort(n: i16) -> Self { Self { short: n } }
    pub fn nint(n: i32) -> Self { Self { int: n } }
    pub fn nlong(n: i64) -> Self { Self { long: n } }

    pub fn nfloat(n: f32) -> Self { Self { float: n } }
    pub fn ndouble(n: f64) -> Self { Self { double: n } }

    pub fn narray(a: *mut Array) -> Self { Self { array: a }}
    pub fn nobject(o: *mut Object) -> Self { Self { object: o }}

    pub fn byte(self) -> i8 { unsafe { self.byte } }
    pub fn short(self) -> i16 { unsafe { self.short } }
    pub fn char(self) -> u16 { unsafe { self.char } }
    pub fn int(self) -> i32 { unsafe { self.int } }
    pub fn long(self) -> i64 { unsafe { self.long } }

    pub fn float(self) -> f32 { unsafe { self.float } }
    pub fn double(self) -> f64 { unsafe { self.double } }

    pub fn array(self) -> *mut Array { unsafe { self.array } }
    pub fn object(self) -> *mut Object { unsafe { self.object } }
}

impl Display for Value {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    f.write_str(&*format!("{:016x}", unsafe { self.long }))
  }
}

impl Debug for Value {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    f.write_str(&*format!("{:016x}", unsafe { self.long }))
  }
}
