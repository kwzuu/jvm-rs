use crate::method::{Method};
use crate::things::Value;

#[derive(Debug)]
pub struct StackFrame {
    pub locals: Vec<Value>,
    pub operand_stack: Vec<Value>,
}

impl StackFrame {
    pub fn new_for(m: &Method) -> StackFrame {
        match m {
            Method::Native(m) => {
                StackFrame {
                    locals: Vec::with_capacity(m.argc()),
                    operand_stack: Vec::with_capacity(0),
                }
            }
            Method::Java(m) => {
                let code = m.code.as_ref().unwrap();
                StackFrame {
                    locals: Vec::with_capacity(code.max_locals as usize),
                    operand_stack: Vec::with_capacity(code.max_stack as usize),
                }
            }
        }

    }
}
