use crate::attributes::code::Code;
use crate::attributes::code_reader::CodeReader;
use crate::constant_pool::ConstantPoolInfo;
use crate::method_info::MethodInfo;
use crate::stack_frame::StackFrame;
use crate::things::Thing;
use crate::{Class, descriptor, Runtime};
use std::collections::HashMap;
use std::rc::Rc;
use crate::bytecode::Instruction;
use crate::constant_pool::representations::MethodHandle;
use crate::things::Thing::Int;

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

    pub fn exec(&self, runtime: &mut Runtime, class: Rc<Class>, stack_frame: &mut StackFrame) -> Option<Thing> {
        println!("{}.{} called", class.this_class, self.name);
        let mut pc: usize = 0;
        let code = &self.code.clone().expect("called method with no code!");
        let get_cpi = |x| {
            class.constant_pool[x as usize - 1].clone()
        };
        loop {
            dbg!(code.code[pc]);
            match code.code[pc] {
                Instruction::Iload0 => stack_frame.operand_stack.push(stack_frame.locals[0].clone()),
                Instruction::Iconst2 => stack_frame.operand_stack.push(Int(2)),
                Instruction::Iconst5 => stack_frame.operand_stack.push(Int(5)),
                Instruction::Imul => {
                    let one = stack_frame.operand_stack.pop().unwrap().int();
                    let two = stack_frame.operand_stack.pop().unwrap().int();
                    stack_frame.operand_stack.push(Int(one * two))
                }
                Instruction::Invokestatic(n) => {
                    let methodref = &class.constant_pool[n as usize - 1]
                        .methodref()
                        .unwrap();

                    let called_class = get_cpi(methodref.class_index)
                        .class()
                        .unwrap();

                    let called_nameandtype = get_cpi(methodref.name_and_type_index)
                        .name_and_type()
                        .unwrap();

                    let cls = &get_cpi(called_class.name_index).utf8().unwrap();
                    let name = &get_cpi(called_nameandtype.name_index).utf8().unwrap();
                    let descriptor = &get_cpi(called_nameandtype.descriptor_index).utf8().unwrap();

                    dbg!(cls);
                    dbg!(name);
                    dbg!(descriptor);

                    let called = runtime.find_method(cls, name, descriptor).unwrap();

                    let mut new_frame = StackFrame::new_for(called.clone());

                    let argc = descriptor::info(&*descriptor).args.len();

                    for _ in 0..argc {
                        new_frame.locals.push(stack_frame.operand_stack.pop().unwrap())
                    }

                    if let Some(x) = called.exec(runtime, class.clone(), &mut new_frame) {
                        stack_frame.operand_stack.push(x)
                    }
                }

                Instruction::Ireturn => return stack_frame.operand_stack.pop(),
                x => panic!("unknown bytecode {:#?}", x)
            }
            pc += 1;
            dbg!(pc);
            dbg!(&stack_frame);
            if pc > code.code.len() {
                panic!("code overrun!")
            }
        }
    }
}
