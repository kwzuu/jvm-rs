use std::ptr::null_mut;
use crate::{Class, Runtime};

pub fn object(runtime: &mut Runtime) {
    runtime.add_class(
        Class {
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