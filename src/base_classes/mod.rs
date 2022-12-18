use std::ptr::null_mut;

pub fn base_classes() -> Vec<Class> {
    let mut classes = vec![];

    let object = Class {
        name: "java/lang/Object".to_string(),
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

    classes.push(object);

    classes
}