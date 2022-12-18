use std::ptr::null_mut;
use crate::{Runtime};
use crate::class::{NativeClass, access_flags::*};

pub fn object(runtime: &mut Runtime) {
    runtime.add_native_class(
        NativeClass {
            name: "java/lang/Object".to_string(),
            access_flags: PUBLIC,
            super_class: null_mut(),
            interfaces: vec![],
            static_fields: Default::default(),
            instance_fields: Default::default(),
            methods: Default::default(),
        }
    )
}