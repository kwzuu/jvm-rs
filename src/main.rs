#![feature(pointer_byte_offsets)]
#![feature(fn_traits)]
#![feature(let_chains)]
#![feature(map_try_insert)]
#![allow(dead_code)]

extern crate core;

use crate::class::JavaClass;
use crate::class_reader::ClassReader;
use crate::runtime::Runtime;

pub mod attributes;
pub mod bytecode;
pub mod class;
pub mod class_file;
pub mod class_reader;
pub mod constant_pool;
pub mod descriptor;
pub mod field_info;
pub mod method;
pub mod method_info;
pub mod runtime;
pub mod values;
pub mod heap;
pub mod base_classes;
pub mod stack;
pub mod thread;
pub mod cli;
pub mod map;
pub mod thread_runtime;
pub mod bcw;
pub mod class_loader;
pub mod settings;

#[macro_export]
macro_rules! os_str {
    ($x:expr) => {$x.to_str().unwrap().to_string()};
}

#[cfg(not(feature = "multithreaded"))]
fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("argument needed")
    }

    let mut runtime = Runtime::new(args[1].clone())?;

    let _ = &mut runtime.run_main();
    Ok(())
}

#[cfg(feature = "multithreaded")]
#[tokio::main]
async fn thread_main() {
    let classloader_bcw = BidirectionalChannelWrapper::<*mut ()>::new();
    if classloader_bcw.id != 0 {
        panic!("bcw_harness.id != 0, can't give class_loader bad id");
    }
    let mut class_loader = ClassLoader::new(classloader_bcw);

    let cl_f = class_loader.harness(vec![]);
}
