use std::collections::HashMap;
use crate::{JavaClass, Runtime};
use crate::class::NativeClass;
use crate::field_info::{AccessHelper, Field};
use crate::stack_frame::StackFrame;
use crate::things::Value;

pub(crate) fn system(runtime: &mut Runtime) {
    let printstream = runtime.get_class("java/io/PrintStream").unwrap();

    let system = NativeClass {
        name: "java/lang/System".to_string(),
        access_flags: 0,
        super_class: runtime.get_class("java/lang/Object").unwrap(),
        interfaces: vec![],
        static_fields: {
            // let new_printstream = unsafe {
            //     (&*printstream).get_method(
            //         "<init>".to_string(),
            //         "(Ljava/lang/String;)V".to_string()
            //     ).unwrap()
            // };
            //
            // let ps_in = {
            //     let mut frame = StackFrame::new_for(new_printstream);
            //     frame.locals.push(runtime.alloc(printstream));
            //     frame.locals.push(runtime.new_string("/dev/stdin"));
            //     new_printstream.exec(
            //         runtime,
            //         printstream,
            //         &mut StackFrame {
            //             locals: vec![],
            //             operand_stack: vec![]
            //         }
            //     )
            // };
            // let ps_in_field = Field {
            //     access_flags: 0,
            //     name: "".to_string(),
            //     descriptor: "".to_string(),
            //     attributes: Default::default(),
            //     access_helper: AccessHelper { value: Value { object: ps_in } },
            // };
            // let ps_out_field = Field {
            //     access_flags: 0,
            //     name: "".to_string(),
            //     descriptor: "".to_string(),
            //     attributes: Default::default(),
            //     access_helper: ()
            // };
            // let ps_err_field = Field {
            //     access_flags: 0,
            //     name: "".to_string(),
            //     descriptor: "".to_string(),
            //     attributes: Default::default(),
            //     access_helper: ()
            // };

            let mut m = HashMap::new();

            // m.insert(("in".to_string(), "Ljava/io/PrintSteam".to_string()), ps_in_field);
            // m.insert(("out".to_string(), "Ljava/io/PrintSteam".to_string()), ps_out_field);
            // m.insert(("err".to_string(), "Ljava/io/PrintSteam".to_string()), ps_err_field);

            m
        },
        instance_fields: Default::default(),
        methods: Default::default(),
    };

    runtime.add_native_class(system);
}