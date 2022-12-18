use crate::{JavaClass, Runtime};

pub fn printstream(runtime: &mut Runtime) {
    let ps = JavaClass {
        name: "java/io/PrintSteam".to_string(),
        constant_pool: vec![],
        access_flags: 0,
        super_class: runtime.get_class("java/io/FilterOutputStream").unwrap(),
        interfaces: vec![
            runtime.get_class("java/lang/Appendable").unwrap(),
            runtime.get_class("java/io/Closeable").unwrap(),
        ],
        static_fields: Default::default(),
        instance_fields: Default::default(),
        methods: Default::default(),
        attributes: Default::default(),
        field_order: vec![]
    };

    runtime.add_class(ps);
}