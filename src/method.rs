use crate::attributes::code::Code;
use crate::attributes::code_reader::CodeReader;
use crate::constant_pool::ConstantPoolInfo;
use crate::method_info::MethodInfo;
use crate::stack_frame::StackFrame;
use crate::things::Thing;
use crate::{Class, Runtime};
use std::collections::HashMap;
use std::rc::Rc;
use crate::bytecode::Instruction;

#[derive(Debug, Clone)]
pub struct Method {
    pub name: String,
    pub access_flags: u16,
    pub descriptor: String,
    pub attributes: HashMap<String, Vec<u8>>,
    pub code: Option<Code>,
}

impl Method {
    pub fn from_info(cp: &Vec<ConstantPoolInfo>, mi: &MethodInfo) -> Method {
        let mut m = Method {
            name: cp[(mi.name_index - 1) as usize]
                .utf8()
                .expect("bad utf8 for method name"),
            access_flags: mi.access_flags,
            attributes: HashMap::new(),
            descriptor: cp[(mi.descriptor_index - 1) as usize]
                .utf8()
                .expect("bad utf8 for method descriptor"),
            code: None,
        };
        for ai in &mi.attributes {
            m.attributes.insert(
                cp[(ai.name_index - 1) as usize].utf8().expect(""),
                ai.info.clone(),
            );
        }
        if let Some(code) = m.attributes.get("Code") {
            m.code = Some(CodeReader::new(code).read_code().expect("code read fail"))
        }
        m
    }

    pub fn exec(
        &self,
        runtime: Rc<Runtime>,
        class: Rc<Class>,
        stack_frame: StackFrame,
    ) -> Option<Thing> {
        let mut stack = vec![];

        println!("{}.{} called", class.this_class, self.name);
        let mut pc: usize = 0;
        let code = &self.code.clone().expect("called method with no code!");
        loop {
            match code.code[pc] {
                Instruction::Iconst2 => stack.push(Thing::Int(2)),
                Instruction::Ireturn => return stack.pop(),
                _ => {}
            }
            pc += 1;
            if pc > code.code.len() {
                panic!("code overrun!")
            }
        }
    }
}
