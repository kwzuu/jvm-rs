use crate::field_info::Field;
use crate::method::Method;
use crate::stack_frame::StackFrame;
use crate::{Class, ClassReader, main};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Runtime<'a> {
    main_class: *mut Class<'a>,
    path_to_main: &'a str,
    loaded_classes: HashMap<String, Class<'a>>, // name:class
}

impl<'a> Runtime<'a> {
    pub fn new(main_class_path: &'a str) -> Result<Runtime, std::io::Error> {
        println!("loading class from {main_class_path}");
        let main_class = Class::from_filename(&main_class_path)?;
        let mut loaded = HashMap::new();
        let name = main_class.name.clone();
        loaded.insert(main_class.name.clone(), main_class);

        let mut runtime = Runtime {
            main_class: loaded.get_mut("").unwrap() as *mut Class,
            path_to_main: main_class_path,
            loaded_classes: HashMap::new(),
        };

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
            ClassReader::new(&(name + ".class"))?.read_classfile(),
            &mut self
        );

        self.loaded_classes.insert(name.to_string(), cls);

        return Ok(self.loaded_classes.get_mut(&name).unwrap() as *mut Class);
    }

    pub fn get_field(&mut self, cls: String, name: &String) -> Result<&'a Field, ()> {
        let loaded = self.load(cls);
        match loaded {
            Ok(c) => unsafe { x.get_field(name) },
            _ => Err(()),
        }
    }

    pub fn find_method(
        &mut self,
        cls: String,
        name: &str,
        descriptor: &str,
    ) -> Result<Rc<Method>, ()> {
        let loaded = self.load(cls);

        if let Ok(c) = loaded {
            return c.get_method(name.to_string(), descriptor.to_string());
        }

        Err(())
    }

    pub fn run_main(self: &mut Self) {
        unsafe {
            let mut main_method = self.main_class->get_method(
                "main".to_string(),
                "([Ljava/lang/String;)V".to_string()
            ).or_else(|()| get_method("main".to_string(), "()I".to_string())
                .or_else(|()| {
                    self.main_class
                        .get_method(
                            "main".to_string(),
                            //&"([Ljava/lang/String;)V".to_string()
                            "()L".to_string(),
                        )
                        .expect("loading method 'main' failed!")
                })).unwrap();
        }

        // frame will later(tm) contain the String[] for the `String[] args`
        let mut frame = StackFrame::new_for(main_method);

        let result = main_method
            .exec(self, self.main_class.clone(), &mut frame)
            .unwrap()
            .int();
        dbg!(result);
    }
}
