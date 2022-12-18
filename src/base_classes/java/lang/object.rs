use std::ptr::null_mut;
use crate::{JavaClass, Runtime};

pub fn object(runtime: &mut Runtime) {
    runtime.add_class(
        JavaClass {
            name: "java/lang/Object".to_string(),
            constant_pool: vec![],
            access_flags: 0,
            super_class: null_mut(),
            interfaces: vec![],
            static_fields: Default::default(),
            instance_fields: Default::default(),
            methods: Default::default(),
            attributes: Default::default(),
            field_order: vec![]
        }
    )
}