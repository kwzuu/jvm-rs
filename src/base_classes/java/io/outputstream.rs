use crate::{JavaClass, Runtime};

pub fn outputstream(runtime: &mut Runtime) {
    let os = JavaClass {
        name: "java/io/OutputStream".to_string(),
        constant_pool: vec![],
        access_flags: 0,
        super_class: runtime.get_class("java/lang/Object").unwrap(),
        interfaces: vec![],
        static_fields: Default::default(),
        instance_fields: Default::default(),
        methods: Default::default(),
        attributes: Default::default(),
        field_order: vec![]
    };

    runtime.add_class(os);
}