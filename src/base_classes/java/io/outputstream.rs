use crate::{Runtime};
use crate::class::NativeClass;

pub fn outputstream(runtime: &mut Runtime) {
    let os = NativeClass {
        name: "java/io/OutputStream".to_string(),
        access_flags: 0,
        super_class: runtime.get_class("java/lang/Object").unwrap(),
        interfaces: vec![],
        static_fields: Default::default(),
        instance_fields: Default::default(),
        methods: Default::default(),
    };

    runtime.add_native_class(os);
}