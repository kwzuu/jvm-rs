extern crate core;

use crate::class::Class;
use crate::class_reader::ClassReader;
use crate::runtime::Runtime;
use std::rc::Rc;

mod attributes;
mod bytecode;
mod class;
mod class_file;
mod class_reader;
mod constant_pool;
mod field_info;
mod method;
mod method_info;
mod runtime;
mod stack_frame;
mod things;
mod descriptor;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("argument needed")
    }
    // dbg!(Class::from_classfile(ClassReader::new(&args[1].clone())?.read_classfile()));
    let mut runtime = Runtime::new(Rc::new(args[1].clone()))?;

    &mut runtime.run_main();
    Ok(())
}
