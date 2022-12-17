
use crate::method::{Method};
use crate::stack_frame::StackFrame;
use crate::{Class, ClassReader};

use std::collections::HashMap;

use crate::descriptor::Type;

pub struct Runtime {
    main_class: *mut Class,
    path_to_main: String,
    loaded_classes: HashMap<String, Class>, // name:class
}

impl Runtime {
    pub fn new(main_class_path: String) -> Result<Runtime, std::io::Error> {
        let mut runtime = Runtime {
            main_class: 0 as *mut Class,
            path_to_main: main_class_path.clone(),
            loaded_classes: HashMap::new(),
        };

        println!("loading builtin classes");
        let classes = crate::base_classes::base_classes();
        for c in classes {
            runtime.loaded_classes.insert(c.name.clone(), c);
        }

        println!("loading class from {main_class_path}");
        let main_class = Class::from_filename(&main_class_path, &mut runtime)?;
        let name = main_class.name.clone();
        runtime.loaded_classes.insert(name.clone(), main_class);
        runtime.main_class = runtime.loaded_classes
            .get_mut(&*name)
            .unwrap() as *mut Class;

        Ok(runtime)
    }

    pub fn load(&mut self, name: String) -> Result<*mut Class, std::io::Error> {
        if let Some(cls) = self.loaded_classes.get_mut(&name) {
            return Ok(cls as *mut Class);
        }

        println!("searching for {name}.class");

        for (loaded, _) in &self.loaded_classes {
            println!("{} is already loaded", loaded)
        }

        let cls = Class::from_classfile(
            ClassReader::new(&(name.clone() + ".class"))?.read_classfile(),
            self
        );

        self.loaded_classes.insert(name.to_string(), cls);

        return Ok(self.loaded_classes.get_mut(&name).unwrap() as *mut Class);
    }

    pub fn run_main<'a>(self: &mut Self) {
        let main_class: &'a Class = unsafe { &*self.main_class };
        let main_string_args: Result<&'a Method, ()> = main_class.get_method(
            "main".to_string(),
            "([Ljava/lang/String;)V".to_string()
        );

        let int_main= |_| main_class.get_method(
            "main".to_string(),
            "()I".to_string()
        );
        let long_main = |_| main_class.get_method(
            "main".to_string(),
            "()L".to_string(),
        );

        let main_method = main_string_args
            .or_else(int_main)
            .or_else(long_main)
            .expect("finding main method failed! checked `static int main()`, \
            `static long main`, `static void main(String[])`");

        // frame will later(tm) contain the String[] for the `String[] args`
        let mut frame = StackFrame::new_for(main_method);
        if frame.locals.capacity() > 0 {
            panic!("static void main(String[]) entry point not yet supported")
        }

        let result = main_method.exec(
            self,
            self.main_class,
            &mut frame
        ).unwrap();

        match main_method.descriptor().ret {
            Type::Int => { dbg!(result.int()); },
            Type::Void => {},
            _ => panic!("unsupported return type!"),
        };
    }
}
