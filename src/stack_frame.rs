use crate::method::Method;
use crate::things::Value;

#[derive(Debug)]
pub struct StackFrame<'a> {
    pub locals: Vec<Value<'a>>,
    pub operand_stack: Vec<Value<'a>>,
}

impl StackFrame {
    pub fn new_for(m: &Method) -> StackFrame {
        StackFrame {
            locals: Vec::with_capacity(m.code.unwrap().max_locals as usize),
            operand_stack: Vec::with_capacity(m.code.unwrap().max_stack as usize),
        }
    }
}
