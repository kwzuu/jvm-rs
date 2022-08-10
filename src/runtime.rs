use crate::field_info::Field;
use crate::method::Method;
use crate::stack_frame::StackFrame;
use crate::{Class, ClassReader};
use std::collections::HashMap;
use std::rc::Rc;

pub struct Runtime<'a> {
    main_class: Rc<Class>,
    loaded: HashMap<&'a String, Rc<Class>>, // name:class
}

impl<'a> Runtime<'a> {
    pub fn new(main_class_name: &'a String) -> Result<Runtime, std::io::Error> {
        let main_class = Class::from_filename(main_class_name)?;
        let mut runtime = Runtime {
            main_class: Rc::new(main_class),
            loaded: HashMap::new(),
        };
        runtime
            .loaded
            .insert(main_class_name, runtime.main_class.clone());
        Ok(runtime)
    }

    pub fn load(&'a mut self, name: &'a String) -> Result<Rc<Class>, std::io::Error> {
        if let Some(cls) = self.loaded.get(name) {
            return Ok(cls.clone());
        }

        let cls = Rc::new(Class::from_classfile(
            ClassReader::new(name)?.read_classfile(),
        ));
        self.loaded.insert(name, cls.clone());
        return Ok(cls);
    }

    pub fn get_field(&'a mut self, cls: &'a String, name: &'a String) -> Result<Rc<Field>, ()> {
        let loaded = self.load(cls);
        match loaded {
            Ok(x) => x.get_field(name),
            _ => Err(()),
        }
    }

    pub fn find_method(
        &'a mut self,
        cls: &'a String,
        name: &'a String,
        descriptor: &'a String,
    ) -> Result<Rc<Method>, ()> {
        let loaded = self.load(cls);
        match loaded {
            Ok(x) => x.get_method(name, descriptor),
            _ => Err(()),
        }
    }

    pub fn run_main(self: Rc<Self>) {
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

        dbg!(main_method
            .exec(self.clone(), self.clone().main_class.clone(), frame,)
            .unwrap()
            .int());
    }
}
