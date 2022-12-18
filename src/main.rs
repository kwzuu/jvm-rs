#![feature(pointer_byte_offsets)]
#![allow(dead_code)]
extern crate core;

use crate::class::JavaClass;
use crate::class_reader::ClassReader;
use crate::runtime::Runtime;

mod attributes;
mod bytecode;
mod class;
mod class_file;
mod class_reader;
mod constant_pool;
mod descriptor;
mod field_info;
mod method;
mod method_info;
mod runtime;
mod stack_frame;
mod things;
mod heap;
mod base_classes;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("argument needed")
    }

    let mut runtime = Runtime::new(args[1].clone())?;

    let _ = &mut runtime.run_main();
    Ok(())
}
