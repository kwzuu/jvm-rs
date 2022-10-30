use std::cell::RefCell;
use crate::field_info::Field;
use crate::method::Method;
use crate::stack_frame::StackFrame;
use crate::{Class, ClassReader};
use std::collections::HashMap;
use std::rc::Rc;

pub struct Runtime {
    main_class: Rc<Class>,
    loaded: HashMap<String, Rc<Class>>, // name:class
}

impl Runtime {
    pub fn new(main_class_name: Rc<String>) -> Result<Runtime, std::io::Error> {
        let main_class = Class::from_filename(&main_class_name)?;
        let mut runtime = Runtime {
            main_class: Rc::new(main_class),
            loaded: HashMap::new(),
        };
        runtime
            .loaded
            .insert((*main_class_name).clone(), runtime.main_class.clone());
        Ok(runtime)
    }

    pub fn load(&mut self, name: String) -> Result<Rc<Class>, std::io::Error> {
        if let Some(cls) = self.loaded.get(&name) {
            return Ok(cls.clone());
        }

        let cls = Rc::new(Class::from_classfile(
            ClassReader::new(&(name.to_string() + &".class".to_string()))?.read_classfile(),
        ));
        self.loaded.insert(name, cls.clone());
        return Ok(cls);
    }

    pub fn get_field(&mut self, cls: String, name: &String) -> Result<Rc<Field>, ()> {
        let loaded = self.load(cls);
        match loaded {
            Ok(x) => x.get_field(name),
            _ => Err(()),
        }
    }

    pub fn find_method(
        &mut self,
        cls: &String,
        name: &String,
        descriptor: &String,
    ) -> Result<Rc<Method>, ()> {
        let loaded = self.load(cls.clone());
        if let Ok(c) = loaded {
            return c.get_method(name, descriptor)
        }
        Err(())
    }

    pub fn run_main(self: &mut Self) {
        let mut main_method = self
            .main_class
            .get_method(
                &"main".to_string(),
                //&"([Ljava/lang/String;)V".to_string()
                &"()I".to_string(),
            )
            .unwrap_or_else(|()| {
                self.main_class
                    .get_method(
                        &"main".to_string(),
                        //&"([Ljava/lang/String;)V".to_string()
                        &"()L".to_string(),
                    )
                    .expect("loading method 'main' failed!")
            });

        // frame will later(tm) contain the String[] for the `String[] args`
        let mut frame = StackFrame::new_for(main_method.clone());

        let result = main_method
            .exec(
                self,
                self.main_class.clone(),
                &mut frame,
            ).unwrap()
            .int();
        dbg!(result);
    }
}
