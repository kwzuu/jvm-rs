use std::ptr::null_mut;
use crate::{JavaClass, Runtime};

pub fn appendable(runtime: &mut Runtime) {
    let a = JavaClass {
        name: "java/lang/Appendable".to_string(),
        constant_pool: vec![],
        access_flags: 0,
        super_class: null_mut(),
        interfaces: vec![],
        static_fields: Default::default(),
        instance_fields: Default::default(),
        methods: Default::default(),
        attributes: Default::default(),
        field_order: vec![]
    };

    runtime.add_class(a);
}