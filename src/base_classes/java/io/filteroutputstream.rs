use crate::{JavaClass, Runtime};

pub fn filteroutputstream(runtime: &mut Runtime) {
    let fos = JavaClass {
        name: "java/io/FilterOutputStream".to_string(),
        constant_pool: vec![],
        access_flags: 0,
        super_class: runtime.get_class("java/io/OutputStream").unwrap(),
        interfaces: vec![],
        static_fields: Default::default(),
        instance_fields: Default::default(),
        methods: Default::default(),
        attributes: Default::default(),
        field_order: vec![]
    };

    runtime.add_class(fos);
}