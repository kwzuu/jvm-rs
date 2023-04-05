use std::collections::HashMap;
use std::ptr::{null_mut};
use crate::{Runtime};
use crate::class::{NativeClass, access_flags::*};
use crate::descriptor::{DescriptorInfo, Type};
use crate::method::{NativeMethod, Method::Native, Method};
use crate::values::Value;

pub fn object(runtime: &mut Runtime) {
    runtime.add_native_class(
        NativeClass {
            name: "java/lang/Object".to_string(),
            access_flags: PUBLIC,
            super_class: null_mut(),
            interfaces: vec![],
            static_fields: Default::default(),
            instance_fields: Default::default(),
            methods: {

                let object_hash_code = Native(NativeMethod {
                    name: "hashCode".to_string(),
                    access_flags: 0,
                    descriptor: "()I".to_string(),
                    parsed_descriptor: DescriptorInfo {
                        ret: Type::Int,
                        args: vec![],
                    },
                    func: Box::new(|_meth, _runtime, _jclass| {
                        println!("hashCode not implemented, returning default value of 0");
                        Some(Value::nint(0))

                    }),
                });
                // equals
                let object_equals = Native(NativeMethod {
                    name: "equals".to_string(),
                    access_flags: 0,
                    descriptor: "(Ljava/lang/Object;)Z".to_string(),
                    parsed_descriptor: DescriptorInfo {
                        ret: Type::Bool,
                        args: vec![Type::Object(Box::from("java/lang/Object".to_string()))],
                    },
                    func: Box::new(|_method, _runtime, _class| {

                        Some(Value::FALSE)
                    }),
                });
                let mut m: HashMap<(String, String), Method> = HashMap::new();

                m.insert(("hashCode".to_string(), "()I".to_string()), object_hash_code);
                m.insert(("equals".to_string(), "(Ljava/lang/Object;)Z".to_string()), object_equals);
                m
            },
        }
    )
}