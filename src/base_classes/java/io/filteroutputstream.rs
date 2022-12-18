use crate::{Runtime};
use crate::class::NativeClass;

pub fn filteroutputstream(runtime: &mut Runtime) {
    let fos = NativeClass {
        name: "java/io/FilterOutputStream".to_string(),
        access_flags: 0,
        super_class: runtime.get_class("java/io/OutputStream").unwrap(),
        interfaces: vec![],
        static_fields: Default::default(),
        instance_fields: Default::default(),
        methods: Default::default(),
    };

    runtime.add_native_class(fos);
}