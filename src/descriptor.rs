use crate::descriptor::Type::{Int, Void};

pub enum Type {
    Int,
    Void
}

pub struct DescriptorInfo {
    ret: Type,
    pub(crate) args: Vec<Type>
}

pub fn info(descriptor: &str) -> DescriptorInfo {
    let mut args = vec![];
    let mut index: usize = 0;
    for i in descriptor.chars() {
        match i {
            '(' => {}
            'I' => args.push(Int),
            'V' => args.push(Void),
            ')' => break,
            _ => {},
        }
        index += 1;
    }
    DescriptorInfo {
        ret: type_from(&descriptor[index+1..]),
        args
    }
}

pub fn type_from(partial_descriptor: &str) -> Type {
    match dbg!(partial_descriptor) {
        "I" => Int,
        "V" => Void,
        _ => panic!("invalid descriptor {}", partial_descriptor)
    }
}