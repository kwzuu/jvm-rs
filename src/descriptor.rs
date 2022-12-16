use crate::descriptor::Type::{Array, Int, Object, Void};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Type {
    Int,
    Void,
    Object(String),
    Array(u8, Box<Type>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DescriptorInfo {
    pub ret: Type,
    pub args: Vec<Type>,
}

fn array_if_nonzero(depth: u8, t: Type) -> Type {
    if depth == 0 {
        t
    } else {
        Array(depth, Box::new(t))
    }
}

pub fn types_from(arg_str: &str) -> Vec<Type> {
    let mut args = vec![];
    let mut array_depth: u8 = 0;
    let mut index: usize = 0;
    while let Some(c) = arg_str.chars().nth(index) {
        index += 1;
        args.push(array_if_nonzero(
            array_depth,
            match c {
                '[' => {
                    array_depth += 1;
                    continue;
                }
                'I' => Int,
                'V' => Void,
                'L' => {
                    let mut s = String::new();
                    while arg_str.chars().nth(index) != Some(';') {
                        s.push(arg_str.chars().nth(index).unwrap());
                        index += 1;
                    }
                    index += 1;
                    Object(s)
                }
                c => panic!("invalid char {}", c),
            },
        ))
    }

    args
}

pub fn args(descriptor: &str) -> Vec<Type> {
    let l_par = descriptor.find('(').unwrap();
    let r_par = descriptor.find(')').unwrap();
    let arg_str = &descriptor[l_par + 1..r_par];

    types_from(arg_str)
}

pub fn info(descriptor: &str) -> DescriptorInfo {
    let mut index: usize = 0;

    DescriptorInfo {
        ret: type_from(&descriptor[index + 1..]),
        args: args(descriptor),
    }
}

pub fn type_from(partial_descriptor: &str) -> Type {
    let v = types_from(partial_descriptor);
    return v[0].clone();
}

mod tests {
    use crate::descriptor::Type::*;
    use crate::descriptor::{info, DescriptorInfo};

    #[test]
    fn test_args() {
        assert_eq!(args("()V"), vec![]);
        assert_eq!(args("(VI)V"), vec![Void, Int]);
        assert_eq!(
            args("(VLjava/lang/String;I)V"),
            vec![Void, Object("java/lang/String".to_string()), Int]
        );
        assert_eq!(
            args("([Ljava/lang/String;)V"),
            vec![Array(1, Box::new(Object("java/lang/String".to_string())))]
        )
    }

    fn test_info() {
        assert_eq!(
            info("([Ljava/lang/String;)V"),
            DescriptorInfo {
                ret: Void,
                args: vec![Array(1, Box::new(Object("java/lang/String".to_string())))]
            }
        )
    }
}
