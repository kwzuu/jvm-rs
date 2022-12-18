use std::ptr::null_mut;
use crate::{Runtime};
use crate::class::NativeClass;

pub fn appendable(runtime: &mut Runtime) {
    let a = NativeClass {
        name: "java/lang/Appendable".to_string(),
        access_flags: 0,
        super_class: null_mut(),
        interfaces: vec![],
        static_fields: Default::default(),
        instance_fields: Default::default(),
        methods: Default::default(),
    };

    runtime.add_native_class(a);
}