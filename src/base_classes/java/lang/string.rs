use std::collections::HashMap;

use crate::{Runtime};
use crate::class::NativeClass;


pub fn string(runtime: &mut Runtime) {
    let s = NativeClass {
        name: "java/lang/String".to_string(),
        access_flags: 0,
        super_class: runtime.get_class("java/lang/Object").unwrap(),
        interfaces: vec![],
        static_fields: Default::default(),
        instance_fields: {
            let m = HashMap::new();

            // m.insert("_data".to_string(), Field {
            //     access_flags: 0,
            //     name: "".to_string(),
            //     descriptor: "".to_string(),
            //     attributes: Default::default(),
            //     access_helper: ()
            // });

            m
        },
        methods: Default::default(),
    };

    runtime.add_native_class(s);
}