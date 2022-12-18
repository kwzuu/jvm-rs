
use crate::method::{Method};
use crate::stack_frame::StackFrame;
use crate::{JavaClass, ClassReader};

use std::collections::HashMap;
use std::ptr::{null, null_mut};

use crate::class::{Class, NativeClass};
use crate::descriptor::Type;
use crate::heap::Heap;
use crate::things::Value;

pub struct Runtime {
    main_class: *mut JavaClass,
    path_to_main: String,
    loaded_classes: HashMap<String, Class>, // name:class
    pub(crate) heap: Heap,
}

impl Runtime {
    pub fn new(main_class_path: String) -> Result<Runtime, std::io::Error> {
        let mut runtime = Runtime {
            main_class: 0 as *mut JavaClass,
            path_to_main: main_class_path.clone(),
            loaded_classes: HashMap::new(),
            heap: Heap::new(),
        };

        println!("loading builtin classes");
        crate::base_classes::base_classes(&mut runtime);

        println!("loading class from {main_class_path}");
        let main_class = JavaClass::from_filename(&main_class_path, &mut runtime)?;
        let name = main_class.name.clone();
        runtime.loaded_classes.insert(name.clone(), Class::Java(main_class));
        runtime.main_class = runtime.loaded_classes
            .get_mut(&*name)
            .unwrap() as *mut JavaClass;

        Ok(runtime)
    }

    // TODO: make strings
    pub fn new_string(&mut self, _contents: &str) -> Value {
        Value::nobject(null_mut())
    }

    pub fn load(&mut self, name: String) -> Result<*mut JavaClass, std::io::Error> {
        if let Some(cls) = self.loaded_classes.get_mut(&name) {
            return Ok(cls as *mut JavaClass);
        }

        println!("searching for {name}.class");

        let cls = JavaClass::from_classfile(
            ClassReader::new(&(name.clone() + ".class"))?.read_classfile(),
            self
        );

        self.add_java_class(cls);

        return Ok(self.loaded_classes.get_mut(&name).unwrap() as *mut JavaClass);
    }

    pub fn run_main<'a>(self: &mut Self) {
        let main_class: &'a JavaClass = unsafe { &*self.main_class };
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

    pub fn add_class(&mut self, cls: Class) {
        self.loaded_classes.insert(cls.name.clone(), cls);
    }

    pub fn add_native_class(&mut self, cls: NativeClass) {
        self.loaded_classes.insert(cls.name.clone(), Class::Native(cls));
    }

    pub fn add_java_class(&mut self, cls: JavaClass) {
        self.loaded_classes.insert(cls.name.clone(), Class::Java(cls));
    }

    pub fn get_class(&mut self, name: &str) -> Result<*mut JavaClass, String> {
        self.loaded_classes.get_mut(name)
            .map(|x| x as *mut JavaClass)
            .ok_or_else(|| name.to_string())
    }
}
