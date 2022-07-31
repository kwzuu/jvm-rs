use std::rc::Rc;
use crate::class::Class;
use crate::class_reader::ClassReader;
use crate::runtime::Runtime;

mod constant_pool;
mod class_reader;
mod class;
mod field_info;
mod method_info;
mod attribute_info;
mod class_file;
mod runtime;
mod bytecode;
mod method;
mod code;
mod code_reader;
mod things;
mod stack_frame;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("argument needed")
    }
    // dbg!(Class::from_classfile(ClassReader::new(&args[1].clone())?.read_classfile()));
    Ok(Rc::new(Runtime::new(&args[1].clone())?).run_main())
}
