use std::rc::Rc;
use crate::method::Method;
use crate::things::Thing;

pub struct StackFrame {
    pub locals: Vec<Thing>,
    pub operand_stack: Vec<Thing>,

}

impl StackFrame {
    pub fn new_for(m: Rc<Method>) -> StackFrame {
        let code = m.code.as_ref().unwrap();
        StackFrame {
            locals: Vec::with_capacity(code.max_locals as usize),
            operand_stack: Vec::with_capacity(code.max_stack as usize)
        }
    }
}