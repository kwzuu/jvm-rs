/*

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use crate::bcw::BidirectionalChannelWrapper;
use crate::class::Class;
use tokio::fs::*;

pub const LOAD_ON_INIT: &[&str] = &[
    "java/lang/Object",
    "java/lang/Class",
    "java/lang/ClassLoader",
    "java/lang/Throwable",
    "java/lang/Exception",
    "java/lang/RuntimeException",
    "java/lang/Cloneable",
    "java/lang/Comparable",
    "java/lang/CharSequence",
    "java/lang/String",
    "java/lang/System",
    "java/lang/Thread",
];

pub struct ClassLoader {
    loaded_classes: HashMap<String, Class>,
    available_classes: HashMap<String, Box<Path>>, // lazy load classes
    bcw: BidirectionalChannelWrapper<*mut ()>
}
impl ClassLoader {
    pub fn new(bcw: BidirectionalChannelWrapper<*mut ()>) -> ClassLoader {
        if bcw.id != 0 {
            panic!("ClassLoader requires bcw.id == 0 to work properly (bcw.id == {})", bcw.id);
        }
        ClassLoader {
            loaded_classes: HashMap::new(),
            available_classes: HashMap::new(),
            bcw
        }
    }

    pub fn load_class(&mut self, name: &str) -> Result<*mut Class, Box<dyn Error>> {
        Ok(match self.loaded_classes.get_mut(name) {
            None => {
                let cls = self.find_class(name)?;
                self.loaded_classes.try_insert(name.to_string(), cls)?
            }
            Some(cls) => cls
        } as *mut Class)
    }

    fn find_class(&self, name: &str) -> Result<Class, Box<dyn Error>> {
        todo!()
    }

    pub async fn harness(&mut self, from: Vec<String>) {
        async fn inside(path: &Path) -> HashMap<String, Box<Path>> {
            let mut m = HashMap::new();

            m
        }

        // todo: load native classes
        for string in from {
            // recursively look for .class files in the given directory
            // if it's one of a few expected classes, load the class
            // otherwise, push the path into the available_classes HashMap

            if Path::new(&string).exists() {
                self.available_classes.extend(inside(&Path::new(&string)).await);
            } else {
                // warn!("{} does not exist", string);
            }
        }
        
        for special in LOAD_ON_INIT {
            if self.available_classes.contains_key(*special) {
                self.load_class(special)
                    .expect("Failure loading init class");
            } else {
                // warn!("{} not found", special);
            }
        }
    }
}
*/