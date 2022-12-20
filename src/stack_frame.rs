use crate::method::{Method};
use crate::things::Value;

struct Stack {
    frames: Vec<StackFrame>,
}

#[derive(Debug)]
pub struct StackFrame {
    locals: Vec<Value>,
    operand_stack: Vec<Value>,
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
                let mut local_count = code.max_locals;
                if !m.is_static() {
                    local_count += 1;
                }
                let null = Value::nlong(0);
                StackFrame {
                    locals: vec![null; local_count as usize],
                    operand_stack: vec![null; code.max_stack as usize],
                }
            }
        }

    }

    pub fn push(&mut self, val: Value) {
        self.operand_stack.push(val);
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.operand_stack.pop()
    }

    pub fn peek(&self) -> Option<Value> {
        self.operand_stack.last().map(Value::clone)
    }

    pub fn peek_n(&self, n: usize) -> Option<Value> {
        self.operand_stack.get(
            self.operand_stack.len() - n
        ).map(Value::clone)
    }

    pub fn get(&self, n: usize) -> Value {
        unsafe { self.locals.get_unchecked(n).clone() }
    }

    pub fn set(&mut self, n: usize, val: Value) {
        unsafe { self.locals.insert(n, val) }
    }
}
