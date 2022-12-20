

use crate::method::{Method};
use crate::stack_frame::StackFrame;
use crate::{JavaClass, ClassReader};

use std::collections::HashMap;
use std::ptr::{null_mut};

use crate::class::{Class, NativeClass};
use crate::descriptor::Type;
use crate::heap::Heap;
use crate::things::Value;

pub const CLASSPATH: &[&str] = &[
    "./",
    "std/class/",
];

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
            .get_mut(&name).unwrap()
            .java_mut().unwrap();

        Ok(runtime)
    }

    // TODO: make strings
    pub fn new_string(&mut self, _contents: &str) -> Value {
        Value::nobject(null_mut())
    }

    pub fn load(&mut self, name: String) -> Result<*mut Class, ()> {
        if let Some(cls) = self.loaded_classes.get_mut(&name) {
            return Ok(cls as *mut Class);
        }

        println!("searching for {name}.class");

        fn get_reader(class_name: &str) -> Option<ClassReader> {
            let paths = CLASSPATH.iter()
                .map(|x| (x.to_string() + class_name + ".class"));

            paths.map(|x| ClassReader::new(&*x)).filter_map(Result::ok).next()
        }

        let cls = JavaClass::from_classfile(
            get_reader(&*name).ok_or(())?.read_classfile(),
            self
        );

        self.add_java_class(cls);

        Ok(self.loaded_classes.get_mut(&name).unwrap() as *mut Class)
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
        let name = cls.name().clone().to_string();
        self.class_merge_in(cls, name);
    }

    pub fn add_native_class(&mut self, cls: NativeClass) {
        self.loaded_classes.insert(cls.name.clone(), Class::Native(cls));
    }

    pub fn add_java_class(&mut self, cls: JavaClass) {
        self.loaded_classes.insert(cls.name.clone(), Class::Java(cls));
    }

    fn class_merge_in(&mut self, cls: Class, name: String) {
        // Generating JavaClass for this because it has more fields :)

        if let Some(first) = self.loaded_classes.get_mut(&name) {
            let mut second = cls;
            if let Class::Java(_) = first && let Class::Java(_) = second {
                panic!("merging JavaClass with JavaClass not yet supported (ERR in loading class {name})");
            }
            match (&first, &second) {
                (Class::Java(_), Class::Native(_)) |
                (Class::Native(_), Class::Native(_)) => {
                    first.merge_methods(&mut second);
                    return;
                },
                (Class::Native(_), Class::Java(_)) => {
                    second.merge_methods(first);
                    self.loaded_classes.insert(name, second);
                    return;
                },
                _ => panic!("merging JavaClass with JavaClass not yet supported (ERR in loading class {name})"),
            }
        } else {
            self.loaded_classes.insert(name, cls);
        }
    }



    pub fn get_class(&mut self, name: &str) -> Result<*mut Class, String> {
        self.loaded_classes.get_mut(name)
            .map(|x| x as *mut Class)
            .ok_or_else(|| name.to_string())
    }
}
