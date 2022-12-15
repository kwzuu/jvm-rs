use crate::field_info::Field;
use crate::method::Method;
use crate::stack_frame::StackFrame;
use crate::{Class, ClassReader};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Runtime {
    main_class: Rc<Class>,
    path_to_main: String,
    loaded_classes: HashMap<String, Rc<Class>>, // name:class
}

impl Runtime {
    pub fn new(main_class_path: Rc<String>) -> Result<Runtime, std::io::Error> {
        println!("loading class from {main_class_path}");
        let main_class = Class::from_filename(&main_class_path)?;
        let mut runtime = Runtime {
            main_class: Rc::new(main_class),
            path_to_main: String::clone(&*main_class_path),
            loaded_classes: HashMap::new(),
        };
        runtime.loaded_classes.insert(
            runtime.main_class.name.clone(),
            runtime.main_class.clone(),
        );
        Ok(runtime)
    }

    pub fn load(&mut self, name: &str) -> Result<Rc<Class>, std::io::Error> {
        if let Some(cls) = self.loaded_classes.get(name) {
            return Ok(cls.clone());
        }

        println!("searching for {name}.class");

        for (loaded, _) in &self.loaded_classes {
            println!("{} is already loaded", loaded)
        }

        let cls = Rc::new(Class::from_classfile(
            ClassReader::new(&(name.to_owned() + ".class"))?.read_classfile(),
        ));

        self.loaded_classes.insert(name.to_owned(), cls.clone());
        return Ok(cls);
    }

    pub fn get_field(&mut self, cls: &str, name: &String) -> Result<Rc<Field>, ()> {
        let loaded = self.load(cls);
        match loaded {
            Ok(x) => x.get_field(name),
            _ => Err(()),
        }
    }

    pub fn find_method(
        &mut self,
        cls: &str,
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
        let mut main_method = self
            .main_class
            .get_method(
                "main".to_string(),
                //&"([Ljava/lang/String;)V".to_string()
                "()I".to_string(),
            )
            .unwrap_or_else(|()| {
                self.main_class
                    .get_method(
                        "main".to_string(),
                        //&"([Ljava/lang/String;)V".to_string()
                        "()L".to_string(),
                    )
                    .expect("loading method 'main' failed!")
            });

        // frame will later(tm) contain the String[] for the `String[] args`
        let mut frame = StackFrame::new_for(main_method.clone());

        let result = main_method
            .exec(self, self.main_class.clone(), &mut frame)
            .unwrap()
            .int();
        dbg!(result);
    }
}
