use crate::{Runtime};
use crate::class::NativeClass;

pub fn printstream(runtime: &mut Runtime) {
    let ps = NativeClass {
        name: "java/io/PrintStream".to_string(),
        access_flags: 0,
        super_class: runtime.get_class("java/io/FilterOutputStream").unwrap(),
        interfaces: vec![
            runtime.get_class("java/lang/Appendable").unwrap(),
            runtime.get_class("java/io/Closeable").unwrap(),
        ],
        static_fields: Default::default(),
        instance_fields: Default::default(),
        methods: Default::default(),
    };

    runtime.add_native_class(ps);
}
