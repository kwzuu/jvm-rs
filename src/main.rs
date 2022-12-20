#![feature(pointer_byte_offsets)]
#![feature(fn_traits)]
#![feature(let_chains)]
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
pub mod stack_frame;
pub mod things;
pub mod heap;
pub mod base_classes;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("argument needed")
    }

    let mut runtime = Runtime::new(args[1].clone())?;

    let _ = &mut runtime.run_main();
    Ok(())
}
