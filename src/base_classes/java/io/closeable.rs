use std::ptr::null_mut;
use crate::{Runtime};
use crate::class::NativeClass;

pub fn closeable(runtime: &mut Runtime) {
    let fos = NativeClass {
        name: "java/io/Closeable".to_string(),
        access_flags: 0,
        super_class: null_mut(),
        interfaces: vec![],
        static_fields: Default::default(),
        instance_fields: Default::default(),
        methods: Default::default(),
    };

    runtime.add_native_class(fos);
}