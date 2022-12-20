use std::collections::HashMap;
use std::ptr::{hash, null_mut};
use crate::{Runtime};
use crate::class::{NativeClass, access_flags::*};
use crate::descriptor::{DescriptorInfo, Type};
use crate::method::{NativeMethod, Method::Native, Method};
use crate::things::Value;

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
                let object_get_class = Native(NativeMethod {
                    name: "getClass".to_string(),
                    access_flags: 0,
                    descriptor: "()Ljava/lang/Class;".to_string(),
                    parsed_descriptor: DescriptorInfo {
                        ret: Type::Object(Box::from("java/lang/Class".to_string())),
                        args: vec![],
                    },
                    func: Box::new(|meth, runtime, jclass| {
                        panic!("not implemented [!TYPEPARAMS]");
                    }),
                });
                let object_hash_code = Native(NativeMethod {
                    name: "hashCode".to_string(),
                    access_flags: 0,
                    descriptor: "()I".to_string(),
                    parsed_descriptor: DescriptorInfo {
                        ret: Type::Int,
                        args: vec![],
                    },
                    func: Box::new(|meth, runtime, jclass| {
                        println!("hashCode not implemented, returning default value of 0");
                        Some(Value::nint(0))

                    }),
                });
                let object_equals = Native(NativeMethod {
                    name: "equals".to_string(),
                    access_flags: 0,
                    descriptor: "(Ljava/lang/Object;)Z".to_string(),
                    parsed_descriptor: DescriptorInfo {
                        ret: Type::,
                        args: vec![Type::Object(Box::from("java/lang/Object".to_string()))],
                    },
                    func: Box::new(|meth, runtime, jclass| {
                        panic!("not implemented [!TYPEPARAMS]");
                    }),
                });
                let mut m: HashMap<(String, String), Method> = HashMap::new();

                m.insert(("getClass".to_string(), "()Ljava/lang/Class;".to_string()), object_get_class);
                m.insert(("hashCode".to_string(), "()I".to_string()), object_hash_code);
                m
            },
        }
    )
}