use crate::class::Class;
use crate::class_reader::ClassReader;

mod constant_pool;
mod class_reader;
mod class;
mod field_info;
mod method_info;
mod attribute_info;
mod class_file;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("argument needed")
    }
    dbg!(Class::new(&ClassReader::new(args[1].clone()).read_class()));
}
