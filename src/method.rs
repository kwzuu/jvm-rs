use std::cmp::Ordering;
use crate::attributes::code::Code;
use crate::attributes::code_reader::CodeReader;
use crate::bytecode::Instruction;

use crate::constant_pool::ConstantPoolInfo;
use crate::method_info::MethodInfo;
use crate::stack_frame::StackFrame;
use crate::things::{Value};
use crate::{descriptor, JavaClass, Runtime};
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
        class: *mut JavaClass,
        stack_frame: &mut StackFrame,
    ) -> Option<Value> {
        match self {
            Method::Native(m) => {
                let f = &m.func;
                f.call((m, runtime, class))
            },
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
    pub func: Box<dyn Fn(&NativeMethod, &mut Runtime, *mut JavaClass) -> Option<Value>>,
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
    pub parsed_descriptor: DescriptorInfo,
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

    pub fn is_static(&self) -> bool {
        self.access_flags | 8 != 0
    }

    pub fn exec(
        &self,
        runtime: &mut Runtime,
        class: *mut JavaClass,
        stack_frame: &mut StackFrame,
    ) -> Option<Value> {
        println!("{}.{} called", unsafe { &*class }.name, self.name);
        let mut pc: usize = 0;
        let code = &self.code.clone().expect("called method with no code!");
        let constant_pool = &unsafe { &*class }.constant_pool;
        let get_cpi = |x: u16| constant_pool[x as usize - 1].clone();


        loop {
            match code.code[pc] {
                // Constants and NOP
                Instruction::Nop => {},
                Instruction::AconstNull => stack_frame.push(Value::NULL),
                Instruction::IconstM1 => stack_frame.push(Value::ICONST_M1),
                Instruction::Iconst0 => stack_frame.push(Value::ICONST_0),
                Instruction::Iconst1 => stack_frame.push(Value::ICONST_1),
                Instruction::Iconst2 => stack_frame.push(Value::ICONST_2),
                Instruction::Iconst3 => stack_frame.push(Value::ICONST_3),
                Instruction::Iconst4 => stack_frame.push(Value::ICONST_4),
                Instruction::Iconst5 => stack_frame.push(Value::ICONST_5),
                Instruction::Lconst0 => stack_frame.push(Value::LCONST_0),
                Instruction::Lconst1 => stack_frame.push(Value::LCONST_1),
                Instruction::Fconst0 => stack_frame.push(Value::FCONST_0),
                Instruction::Fconst1 => stack_frame.push(Value::FCONST_1),
                Instruction::Fconst2 => stack_frame.push(Value::FCONST_2),
                Instruction::Dconst0 => stack_frame.push(Value::DCONST_0),
                Instruction::Dconst1 => stack_frame.push(Value::DCONST_1),

                // Load locals onto the operand stack
                Instruction::Bipush(x) => stack_frame.push(Value::nint(x as i32)),
                Instruction::Sipush(x) => stack_frame.push(Value::nint(x as i32)),
                Instruction::Ldc(x) => {
                    let cpi = get_cpi(x as u16);
                    stack_frame.push(
                        Value::from(cpi)
                    )
                },
                Instruction::LdcW(x) => {
                    let cpi = get_cpi(x);
                    stack_frame.push(
                        Value::from(cpi)
                    )
                },
                Instruction::Ldc2W(x) => {
                    let cpi = get_cpi(x);
                    stack_frame.push(
                        Value::from(cpi)
                    )
                },
                Instruction::Iload(x) => stack_frame.push(stack_frame.get(x as usize)),
                Instruction::Lload(x) => stack_frame.push(stack_frame.get(x as usize)),
                Instruction::Fload(x) => stack_frame.push(stack_frame.get(x as usize)),
                Instruction::Dload(x) => stack_frame.push(stack_frame.get(x as usize)),
                Instruction::Aload(x) => stack_frame.push(stack_frame.get(x as usize)),

                Instruction::Iload0 => stack_frame.push(stack_frame.get(0)),
                Instruction::Iload1 => stack_frame.push(stack_frame.get(1)),
                Instruction::Iload2 => stack_frame.push(stack_frame.get(2)),
                Instruction::Iload3 => stack_frame.push(stack_frame.get(3)),

                Instruction::Lload0 => stack_frame.push(stack_frame.get(0)),
                Instruction::Lload1 => stack_frame.push(stack_frame.get(1)),
                Instruction::Lload2 => stack_frame.push(stack_frame.get(2)),
                Instruction::Lload3 => stack_frame.push(stack_frame.get(3)),

                Instruction::Fload0 => stack_frame.push(stack_frame.get(0)),
                Instruction::Fload1 => stack_frame.push(stack_frame.get(1)),
                Instruction::Fload2 => stack_frame.push(stack_frame.get(2)),
                Instruction::Fload3 => stack_frame.push(stack_frame.get(3)),

                Instruction::Dload0 => stack_frame.push(stack_frame.get(0)),
                Instruction::Dload1 => stack_frame.push(stack_frame.get(1)),
                Instruction::Dload2 => stack_frame.push(stack_frame.get(2)),
                Instruction::Dload3 => stack_frame.push(stack_frame.get(3)),

                Instruction::Aload0 => stack_frame.push(stack_frame.get(0)),
                Instruction::Aload1 => stack_frame.push(stack_frame.get(1)),
                Instruction::Aload2 => stack_frame.push(stack_frame.get(2)),
                Instruction::Aload3 => stack_frame.push(stack_frame.get(3)),
                Instruction::Iaload => {
                    todo!("arrays no worky");
                },
                Instruction::Laload => {
                    todo!("arrays no worky");
                },
                Instruction::Faload => {
                    todo!("arrays no worky");
                },
                Instruction::Daload => {
                    todo!("arrays no worky");
                },
                
                // arithmetic
                Instruction::Iadd => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nint(a.int() + b.int()))
                },
                Instruction::Ladd => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nlong(a.long() + b.long()))
                },
                Instruction::Fadd => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nfloat(a.float() + b.float()))
                },
                Instruction::Dadd => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::ndouble(a.double() + b.double()))
                },
                Instruction::Isub => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nint(a.int() - b.int()))
                },
                Instruction::Lsub => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nlong(a.long() - b.long()))
                },
                Instruction::Fsub => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nfloat(a.float() - b.float()))
                },
                Instruction::Dsub => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::ndouble(a.double() - b.double()))
                },
                Instruction::Imul => {
                    let one = stack_frame.pop().unwrap().int();
                    let two = stack_frame.pop().unwrap().int();
                    stack_frame.push(Value::nint(one * two))
                },
                Instruction::Lmul => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nlong(a.long() * b.long()))
                },
                Instruction::Fmul => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nfloat(a.float() * b.float()))
                },
                Instruction::Dmul => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::ndouble(a.double() * b.double()))
                },
                Instruction::Idiv => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nint(a.int() / b.int()))
                },
                Instruction::Ldiv => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nlong(a.long() / b.long()))
                },
                Instruction::Fdiv => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nfloat(a.float() / b.float()))
                },
                Instruction::Ddiv => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::ndouble(a.double() / b.double()))
                },
                Instruction::Irem => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nint(a.int() % b.int()))
                },
                Instruction::Lrem => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nlong(a.long() % b.long()))
                },
                Instruction::Frem => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nfloat(a.float() % b.float()))
                },
                Instruction::Drem => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::ndouble(a.double() % b.double()))
                },
                Instruction::Ineg => {
                    let a = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nint(-a.int()))
                },
                Instruction::Lneg => {
                    let a = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nlong(-a.long()))
                },
                Instruction::Fneg => {
                    let a = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nfloat(-a.float()))
                },
                Instruction::Dneg => {
                    let a = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::ndouble(-a.double()))
                },
                Instruction::Ishl => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nint(a.int() << b.int()))
                },
                Instruction::Lshl => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nlong(a.long() << b.long()))
                },
                Instruction::Ishr => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nint(a.int() >> b.int()))
                },
                Instruction::Lshr => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nlong(a.long() >> b.long()))
                },
                Instruction::Iushr => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nint(a.int() >> b.int()))
                },
                Instruction::Lushr => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nlong(a.long() >> b.long()))
                },
                Instruction::Iand => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nint(a.int() & b.int()))
                },
                Instruction::Land => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nlong(a.long() & b.long()))
                },
                Instruction::Ior => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nint(a.int() | b.int()))
                },
                Instruction::Lor => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nlong(a.long() | b.long()))
                },
                Instruction::Ixor => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nint(a.int() ^ b.int()))
                },
                Instruction::Lxor => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nlong(a.long() ^ b.long()))
                },
                Instruction::Iinc(index, n) => {
                    let old = stack_frame.get(index as usize);
                    let new = old.int() + (n as i32);
                    stack_frame.set(index as usize, Value::nint(new));
                },
                Instruction::I2l => {
                    let a = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nlong(a.int() as i64))
                },
                Instruction::I2f => {
                    let a = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nfloat(a.int() as f32))
                },
                Instruction::I2d => {
                    let a = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::ndouble(a.int() as f64))
                },
                Instruction::L2i => {
                    let a = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nint(a.long() as i32))
                },
                Instruction::L2f => {
                    let a = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nfloat(a.long() as f32))
                },
                Instruction::L2d => {
                    let a = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::ndouble(a.long() as f64))
                },
                Instruction::F2i => {
                    let a = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nint(a.float() as i32))
                },
                Instruction::F2l => {
                    let a = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nlong(a.float() as i64))
                },
                Instruction::F2d => {
                    let a = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::ndouble(a.float() as f64))
                },
                Instruction::D2i => {
                    let a = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nint(a.double() as i32))
                },
                Instruction::D2l => {
                    let a = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nlong(a.double() as i64))
                },
                Instruction::D2f => {
                    let a = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nfloat(a.double() as f32))
                },
                Instruction::I2b => {
                    let a = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nint(a.int() as i8 as i32))
                },
                Instruction::I2c => {
                    let a = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nint(a.int() as u8 as i32))
                },
                Instruction::I2s => {
                    let a = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nint(a.int() as i16 as i32))
                },
                Instruction::Lcmp => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nint(a.long().cmp(&b.long()).into_int()))
                },
                Instruction::Fcmpl => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nint(a.float().partial_cmp(&b.float()).unwrap_or(Ordering::Less).into_int()))
                },
                Instruction::Fcmpg => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nint(a.float().partial_cmp(&b.float()).unwrap_or(Ordering::Greater).into_int()))
                },
                Instruction::Dcmpl => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nint(a.double().partial_cmp(&b.double()).unwrap_or(Ordering::Less).into_int()))
                },
                Instruction::Dcmpg => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    stack_frame.push(Value::nint(a.double().partial_cmp(&b.double()).unwrap_or(Ordering::Greater).into_int()))
                },
                Instruction::Ifeq(offset) => {
                    let a = stack_frame.pop().expect("no value on stack");
                    if a.int() == 0 {
                        pc += offset as usize;
                    }
                    continue;
                },
                Instruction::Ifne(offset) => {
                    let a = stack_frame.pop().expect("no value on stack");
                    if a.int() != 0 {
                        pc += offset as usize;
                    }
                    continue;
                },
                Instruction::Iflt(offset) => {
                    let a = stack_frame.pop().expect("no value on stack");
                    if a.int() < 0 {
                        pc += offset as usize;
                    }
                    continue;
                },
                Instruction::Ifge(offset) => {
                    let a = stack_frame.pop().expect("no value on stack");
                    if a.int() >= 0 {
                        pc += offset as usize;
                    }
                    continue;
                },
                Instruction::Ifgt(offset) => {
                    let a = stack_frame.pop().expect("no value on stack");
                    if a.int() > 0 {
                        pc += offset as usize;
                    }
                    continue;
                },
                Instruction::Ifle(offset) => {
                    let a = stack_frame.pop().expect("no value on stack");
                    if a.int() <= 0 {
                        pc += offset as usize;
                    }
                    continue;
                },
                Instruction::IfIcmpeq(offset) => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    if a.int() == b.int() {
                        pc += offset as usize;
                    }
                    continue;
                },
                Instruction::IfIcmpne(offset) => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    if a.int() != b.int() {
                        pc += offset as usize;
                    }
                    continue;
                },
                Instruction::IfIcmplt(offset) => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    if a.int() < b.int() {
                        pc += offset as usize;
                    }
                    continue;
                },
                Instruction::IfIcmpge(offset) => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    if a.int() >= b.int() {
                        pc += offset as usize;
                    }
                    continue;
                },
                Instruction::IfIcmpgt(offset) => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    if a.int() > b.int() {
                        pc += offset as usize;
                    }
                    continue;
                },
                Instruction::IfIcmple(offset) => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    if a.int() <= b.int() {
                        pc += offset as usize;
                    }
                    continue;
                },
                Instruction::IfAcmpeq(offset) => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    if a.int() == b.int() {
                        pc += offset as usize;
                    }
                    continue;
                },
                Instruction::IfAcmpne(offset) => {
                    let a = stack_frame.pop().expect("no value on stack");
                    let b = stack_frame.pop().expect("no value on stack");
                    if a.int() != b.int() {
                        pc += offset as usize;
                    }
                    continue;
                },
                Instruction::Goto(offset) => {
                    pc += offset as usize;
                },
                Instruction::Tableswitch => {
                    todo!()
                },
                Instruction::Lookupswitch => {
                    todo!()
                },
                Instruction::Ireturn |
                Instruction::Lreturn |
                Instruction::Freturn |
                Instruction::Dreturn |
                Instruction::Areturn => return stack_frame.pop(),
                Instruction::Return => return None,

                // gets a static field of a class
                Instruction::Getstatic(n) => {
                    let field = get_cpi(n).fieldref().unwrap();
                    let cls_name = get_cpi(field.class_index).class_name(constant_pool);
                    dbg!(&cls_name);
                    let cls = runtime.load(cls_name).unwrap();

                    let nt = get_cpi(field.name_and_type_index)
                        .name_and_type().unwrap();

                    let name = get_cpi(nt.name_index).utf8().unwrap();

                    let val = unsafe { &*cls }.get_static(&name).unwrap();
                    stack_frame.push(val);
                },
                Instruction::Putstatic(n) => {
                    let field = get_cpi(n).fieldref().unwrap();
                    let cls_name = get_cpi(field.class_index).class_name(constant_pool);
                    dbg!(&cls_name);
                    let cls = runtime.load(cls_name).unwrap();

                    let nt = get_cpi(field.name_and_type_index)
                        .name_and_type().unwrap();

                    let name = get_cpi(nt.name_index).utf8().unwrap();

                    let val = stack_frame.pop().unwrap();
                    unsafe { &mut *cls }.set_static(&mut name.clone(), val);
                },
                Instruction::Getfield(n) => {
                    // objectref is on the stack
                    let field = get_cpi(n).fieldref().unwrap();
                    let obj = stack_frame.pop().unwrap().object();
                    let cls_name = get_cpi(field.class_index).class_name(constant_pool);
                    let cls = runtime.load(cls_name).unwrap();
                    unsafe {
                        let cls_ = &*cls;
                        let obj_ = obj.as_mut().unwrap();
                        let name = get_cpi(get_cpi(field.name_and_type_index)
                            .name_and_type().unwrap().name_index).utf8().unwrap();
                        stack_frame.push( cls_.get_instance_field(obj_, &name) );
                    }
                },
                Instruction::Putfield(n) => {
                    // objectref is on the stack
                    let field = get_cpi(n).fieldref().unwrap();
                    let obj = stack_frame.pop().unwrap().object();
                    let cls_name = get_cpi(field.class_index).class_name(constant_pool);
                    let cls = runtime.load(cls_name).unwrap();
                    unsafe {
                        let cls_ = &mut *cls;
                        let obj_ = obj.as_mut().unwrap();
                        let name = get_cpi(get_cpi(field.name_and_type_index)
                            .name_and_type().unwrap().name_index).utf8().unwrap();
                        let val = stack_frame.pop().unwrap();
                        cls_.set_instance_field(obj_, &name, val);
                    }
                },

                // invoke*
                Instruction::Invokevirtual(n) => {
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

                    for i in 0..argc {
                        new_frame.set(i, stack_frame.pop().unwrap())
                    }

                    if let Some(x) = called.exec(runtime, class.clone(), &mut new_frame) {
                        stack_frame.push(x)
                    }
                },

                Instruction::Invokespecial(n) => {
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

                    for i in 0..argc {
                        new_frame.set(i, stack_frame.pop().unwrap())
                    }

                    if let Some(x) = called.exec(runtime, class.clone(), &mut new_frame) {
                        stack_frame.push(x)
                    }
                },
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
                        new_frame.push(stack_frame.pop().unwrap())
                    }

                    if let Some(x) = called.exec(runtime, class.clone(), &mut new_frame) {
                        stack_frame.push(x)
                    }
                },
                Instruction::Invokeinterface(n) => {
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

                    for i in 0..argc {
                        new_frame.set(i, stack_frame.pop().unwrap())
                    }

                    if let Some(x) = called.exec(runtime, class.clone(), &mut new_frame) {
                        stack_frame.push(x)
                    }
                },
                Instruction::Invokedynamic(n) => {
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

                    for i in 0..argc {
                        new_frame.set(i, stack_frame.pop().unwrap())
                    }

                    if let Some(x) = called.exec(runtime, class.clone(), &mut new_frame) {
                        stack_frame.push(x)
                    }
                },
                Instruction::New(_n) => {
                    todo!()
                },
                Instruction::Newarray(_n) => {
                    todo!()
                },
                Instruction::Anewarray(_n) => {
                    todo!()
                },
                Instruction::Arraylength => {
                    todo!()
                },
                Instruction::Athrow => {
                    todo!()
                },
                Instruction::Checkcast(_n) => {
                    todo!()
                },
                Instruction::Instanceof(_n) => {
                    todo!()
                },
                Instruction::Monitorenter => {
                    todo!()
                },
                Instruction::Monitorexit => {
                    todo!()
                },
                Instruction::Wide3(_n, _m) => {
                    todo!()
                },
                Instruction::Wide5(_n, _m, _o) => {
                    todo!()
                },
                Instruction::Multianewarray(_n, _m) => {
                    todo!()
                },
                Instruction::Ifnull(_n) => {
                    todo!()
                },
                Instruction::Ifnonnull(_n) => {
                    todo!()
                },
                Instruction::GotoW(_n) => {
                    todo!()
                },
                Instruction::Breakpoint => {
                    eprintln!("BREAKPOINT! at {}", pc);
                    eprintln!("Previous instruction: {:?}", code.code[pc - 1]);
                    eprintln!("Next instruction: {:?}", code.code[pc + 1]);

                    dbg!(&stack_frame);
                    // hold for user input
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();

                },
                Instruction::Impdep1 => {
                    panic!("IMPDEP should not appear in classfiles")
                },
                Instruction::Impdep2 => {
                    panic!("IMPDEP should not appear in classfiles")
                },
                x => panic!("unknown bytecode {:#?}, stackframe: {:?}", x, stack_frame),
            }
            pc += 1;
            if pc > code.code.len() {
                panic!("code overrun!")
            }
        }
    }
}

trait IntoInt {
    fn into_int(self) -> i32;
}
impl IntoInt for Ordering {
    fn into_int(self) -> i32 {
        match self {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }
}
