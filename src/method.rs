use crate::attributes::code::Code;
use crate::attributes::code_reader::CodeReader;
use crate::bytecode::Instruction;

use crate::constant_pool::ConstantPoolInfo;
use crate::method_info::MethodInfo;
use crate::stack_frame::StackFrame;
use crate::things::Value;
use crate::{descriptor, Class, Runtime};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::descriptor::DescriptorInfo;

#[derive(Debug)]
pub enum Method {
    Native(NativeMethod),
    Java(JavaMethod),
}

impl Method {
    pub fn from_info(cp: &Vec<ConstantPoolInfo>, mi: &MethodInfo) -> Method {
        Method::Java(JavaMethod::from_info(cp, mi))
    }

    pub fn exec(
        &self,
        runtime: &mut Runtime,
        class: *mut Class,
        stack_frame: &mut StackFrame,
    ) -> Option<Value> {
        match self {
            Method::Native(m) => (m.func)(m, runtime, class),
            Method::Java(m) => m.exec(runtime, class, stack_frame)
        }
    }

    pub fn descriptor(&self) -> &DescriptorInfo {
        match self {
            Method::Native(m) => &m.parsed_descriptor,
            Method::Java(m) => &m.parsed_descriptor,
        }
    }
}

pub struct NativeMethod {
    pub name: String,
    pub access_flags: u16,
    pub descriptor: String,
    pub parsed_descriptor: DescriptorInfo,
    pub func: Box<dyn Fn(&NativeMethod, &mut Runtime, *mut Class) -> Option<Value>>,
}

impl Debug for NativeMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*self.descriptor)?;
        f.write_str(&*self.name)
    }
}

impl NativeMethod {
    pub fn argc(&self) -> usize {
        self.parsed_descriptor.args.len()
    }
}

#[derive(Debug, Clone)]
pub struct JavaMethod {
    pub name: String,
    pub access_flags: u16,
    pub descriptor: String,
    pub parsed_descriptor: descriptor::DescriptorInfo,
    pub attributes: HashMap<String, Vec<u8>>,
    pub code: Option<Code>,
}

impl JavaMethod {
    pub fn from_info(cp: &Vec<ConstantPoolInfo>, mi: &MethodInfo) -> JavaMethod {
        let desc = cp[(mi.descriptor_index - 1) as usize]
            .utf8().expect("bad utf8 for method descriptor");
        let mut m = JavaMethod {
            name: cp[(mi.name_index - 1) as usize]
                .utf8()
                .expect("bad utf8 for method name"),
            access_flags: mi.access_flags,
            attributes: HashMap::new(),
            code: None,
            parsed_descriptor: descriptor::info(&*desc),
            descriptor: desc,
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
        runtime: &mut Runtime,
        class: *mut Class,
        stack_frame: &mut StackFrame,
    ) -> Option<Value> {
        println!("{}.{} called", unsafe { &*class }.name, self.name);
        let mut pc: usize = 0;
        let code = &self.code.clone().expect("called method with no code!");
        let get_cpi = |x| unsafe { &*class }.constant_pool[x as usize - 1].clone();
        loop {
            match code.code[pc] {
                Instruction::Iload0 => stack_frame
                    .operand_stack
                    .push(stack_frame.locals[0].clone()),
                Instruction::Iconst0 => stack_frame.operand_stack.push(Value::ICONST_0),
                Instruction::Iconst2 => stack_frame.operand_stack.push(Value::ICONST_2),
                Instruction::Iconst5 => stack_frame.operand_stack.push(Value::ICONST_5),
                Instruction::Imul => {
                    let one = stack_frame.operand_stack.pop().unwrap().int();
                    let two = stack_frame.operand_stack.pop().unwrap().int();
                    stack_frame.operand_stack.push(Value::nint(one * two))
                }
                Instruction::Invokestatic(n) => {
                    let methodref = get_cpi(n).methodref().unwrap();

                    let called_class = get_cpi(methodref.class_index).class().unwrap();

                    let called_nameandtype = get_cpi(methodref.name_and_type_index)
                        .name_and_type()
                        .unwrap();

                    let cls_name = get_cpi(called_class.name_index).utf8().unwrap();
                    let name = get_cpi(called_nameandtype.name_index).utf8().unwrap();
                    let descriptor = get_cpi(called_nameandtype.descriptor_index).utf8().unwrap();

                    let cls = unsafe { &*runtime.load(cls_name).unwrap() };
                    let called = cls.get_method(name, descriptor.clone()).unwrap();

                    let mut new_frame = StackFrame::new_for(called);

                    let argc = descriptor::info(&*descriptor).args.len();

                    for _ in 0..argc {
                        new_frame
                            .locals
                            .push(stack_frame.operand_stack.pop().unwrap())
                    }

                    if let Some(x) = called.exec(runtime, class.clone(), &mut new_frame) {
                        stack_frame.operand_stack.push(x)
                    }
                }

                Instruction::Ireturn => return stack_frame.operand_stack.pop(),
                x => panic!("unknown bytecode {:#?}", x),
            }
            pc += 1;
            if pc > code.code.len() {
                panic!("code overrun!")
            }
        }
    }
}
