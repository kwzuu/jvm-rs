use crate::{Class, Runtime};

pub(crate) fn system(runtime: &mut Runtime) -> Class {
    Class {
        name: "java/lang/System".to_string(),
        constant_pool: vec![],
        access_flags: 0,
        super_class: runtime.get_class("java/lang/Object").unwrap(),
        interfaces: vec![],
        static_fields: Default::default(),
        instance_fields: Default::default(),
        methods: Default::default(),
        attributes: Default::default(),
        field_order: vec![]
    }
}