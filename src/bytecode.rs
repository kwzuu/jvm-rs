use std::collections::HashMap;
use std::mem;
use std::ops::{ControlFlow, FromResidual, Try};
use lazy_static::lazy_static;
use crate::bytecode::BytecodeParseError::{EarlyEnd, InvalidCode, InvalidOpcode};
use crate::bytecode::Instruction::{AconstNull, Aload, Goto, IfIcmpeq, IfIcmpge, IfIcmpgt, IfIcmple, IfIcmplt, IfIcmpne, Iinc, Iload, Imul, Invokespecial, Ipush, Ireturn, Istore, Nop, Return};

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
/// JVM SE7 Bytecode Instructions
pub enum Instruction {
    /// Do nothing, I suppose.
    Nop,
    AconstNull,
    Ipush(i16),
    Ldc(u32),
    Iload(u16),
    Lload(u16),
    Fload(u16),
    Dload(u16),
    Aload(u16),
    Iaload,
    Laload,
    Faload,
    Daload,
    /// Find item at index of array by popping arrayref
    /// and index from the operand stack, and then push it
    /// to the operand stack.
    Aaload,
    Baload,
    Caload,
    Saload,
    Istore(u16),
    Lstore(u16),
    Fstore(u16),
    Dstore(u16),
    Astore(u16),
    Iastore,
    Lastore,
    Fastore,
    Dastore,
    Aastore,
    Bastore,
    Castore,
    Sastore,
    Pop,
    Pop2,
    Dup,
    DupX1,
    DupX2,
    Dup2,
    Dup2X1,
    Dup2X2,
    Swap,
    Iadd,
    Ladd,
    Fadd,
    Dadd,
    Isub,
    Lsub,
    Fsub,
    Dsub,
    Imul,
    Lmul,
    Fmul,
    Dmul,
    Idiv,
    Ldiv,
    Fdiv,
    Ddiv,
    Irem,
    Lrem,
    Frem,
    Drem,
    Ineg,
    Lneg,
    Fneg,
    Dneg,
    Ishl,
    Lshl,
    Ishr,
    Lshr,
    Iushr,
    Lushr,
    Iand,
    Land,
    Ior,
    Lor,
    Ixor,
    Lxor,
    Iinc(u8, i8),
    I2l,
    I2f,
    I2d,
    L2i,
    L2f,
    L2d,
    F2i,
    F2l,
    F2d,
    D2i,
    D2l,
    D2f,
    I2b,
    I2c,
    I2s,
    Lcmp,
    Fcmpl,
    Fcmpg,
    Dcmpl,
    Dcmpg,
    Ifeq(u16),
    Ifne(u16),
    Iflt(u16),
    Ifge(u16),
    Ifgt(u16),
    Ifle(u16),
    IfIcmpeq(u16),
    IfIcmpne(u16),
    IfIcmplt(u16),
    IfIcmpge(u16),
    IfIcmpgt(u16),
    IfIcmple(u16),
    IfAcmpeq(u16),
    IfAcmpne(u16),
    Goto(u16),
    Tableswitch,  // TODO: implement
    Lookupswitch, // TODO: implement
    Ireturn,
    Lreturn,
    Freturn,
    Dreturn,
    Areturn,
    Return,
    Getstatic(u16),
    Putstatic(u16),
    Getfield(u16),
    Putfield(u16),
    Invokevirtual(u16),
    Invokespecial(u16),
    Invokestatic(u16),
    Invokeinterface(u16),
    Invokedynamic(u16),
    New(u16),
    Newarray(u8),
    Anewarray(u16),
    Arraylength,
    Athrow,
    Checkcast(u16),
    Instanceof(u16),
    Monitorenter,
    Monitorexit,
    Multianewarray(u16, u8),
    Ifnull(u16),
    Ifnonnull(u16),
    Breakpoint,
    Impdep1,
    Impdep2,
}

#[derive(Clone, Debug)]
pub enum BytecodeParseError {
    EarlyEnd,
    InvalidOpcode(u8),
    InvalidCode,
}

impl Instruction {
    /// gets the size of the first instruction stored in `buf`
    /// returns 0 in the case of an invalid opcode or bad formatting
    pub fn raw_size(buf: &[u8]) -> u32 {
        lazy_static! {
            pub static ref TABLE: [u32; 256] = {
                let mut t = [0u32; 256];

                const ZERO_OPERAND: &[u8] = &[
                    // nop
                    0x00,
                    // Xconst_Y
                    0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
                    0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c,
                    0x0d, 0x0e, 0x0f,
                    // Xload_Y
                    0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
                    0x20, 0x21, 0x22, 0x23, 0x24, 0x25,
                    0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b,
                    0x2c, 0x2d,
                    // Xaload
                    0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33,
                    0x34, 0x35,
                    // Xstore_Y
                    0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40,
                    0x41, 0x42, 0x43, 0x44, 0x45, 0x46,
                    0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c,
                    0x4d, 0x4e,
                    // Xastore
                    0x4f, 0x50, 0x51, 0x52, 0x53, 0x54,
                    0x55, 0x56,
                    // stack manipulation
                    0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c,
                    0x5d, 0x5e, 0x5f,
                    // math
                    0x60, 0x61, 0x62, 0x63, 0x64, 0x65,
                    0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b,
                    0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71,
                    0x72, 0x73, 0x74, 0x75, 0x76, 0x77,
                    0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d,
                    0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83,
                    // conversions
                    0x85, 0x86, 0x87, 0x88, 0x89, 0x8a,
                    0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90,
                    0x91, 0x92, 0x93,
                    // comparisons
                    0x94, 0x95, 0x96, 0x97, 0x98,
                    // return
                    0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1,
                    // arraylength
                    0xbe,
                    // athrow
                    0xbf,
                    // monitor
                    0xc2, 0xc3,
                    // breakpoint
                    0xca,
                    // impdep
                    0xfe, 0xff
                ];

                const ONE_OPERAND: &[u8] = &[
                    // bipush
                    0x10,
                    // ldc
                    0x12,
                    // Xload
                    0x15, 0x16, 0x17, 0x18, 0x19,
                    // Xstore
                    0x36, 0x37, 0x38, 0x39, 0x3a,
                    // ret
                    0xa9,
                    // newarray
                    0xbc
                ];

                const TWO_OPERAND: &[u8] = &[
                    // sipush
                    0x11,
                    // ldc_w
                    0x13, 0x14,
                    // iinc
                    0x84,
                    // branches
                    0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e,
                    0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4,
                    0xa5, 0xa6, 0xa7, 0xa8,
                    // fields, statics, invokes
                    0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7,
                    0xb8, 0xb9, 0xba,
                    // new, newarray, checkcast, instanceof
                    0xbb, 0xbd, 0xc0, 0xc1
                ];

                for b in ZERO_OPERAND {
                    t[*b as usize] = 1
                }

                for b in ONE_OPERAND {
                    t[*b as usize] = 2
                }

                for b in TWO_OPERAND {
                    t[*b as usize] = 3
                }

                t[0xc5] = 4; // multinewarray
                t[0xb9] = 5; // invokeinterface
                t[0xba] = 5; // invokedynamic
                t[0xc8] = 5; // goto_w
                t[0xc9] = 5; // jsr_w

                t
            };
        }

        let opcode = {
            let o = buf.get(0);
            if let Some(x) = o {
                *x
            } else {
                return 0
            }
        };

        let table_result = TABLE[opcode as usize];
        if table_result == 0 {
            match opcode {
                0xaa | 0xab => unimplemented!("switches are hard :("),
                0xc4 => {
                    // wide
                    let second_opcode = buf.get(1);
                    if let Some(val) = second_opcode {
                        if *val == 0x84 {
                            // iinc
                            5
                        } else {
                            // other opcode
                            3
                        }
                    } else {
                        0
                    }
                }
                _ => 0
            }
        } else {
            table_result
        }
    }

    pub fn read_from(buf: &[u8], length: u32) -> Result<Vec<Instruction>, BytecodeParseError> {
        dbg!(buf);

        let mut code = vec![];

        let byte_offset_table: HashMap<u32, u16> = {
            let mut m = HashMap::new();
            let mut byte_offset: u32 = 0;
            let mut instruction_index = 0;

            // first pass: get offsets of instructions
            while byte_offset < length {
                m.insert(byte_offset, instruction_index);
                let size = Instruction::raw_size(&buf[byte_offset as usize..]);
                if size == 0 {
                    return Err(InvalidOpcode(buf[byte_offset as usize]))
                }
                byte_offset += size;
                instruction_index += 1;
            }
            m
        };

        let instruction_index_table = {
            let mut m = HashMap::new();
            for (k, v) in byte_offset_table.iter() {
                m.insert(*v, *k);
            }
            m
        };

        // second pass: do the parsing
        // :)

        let mut walker = buf;

        macro_rules! read_u1 {
            () => {{
                let x = *walker.get(0).ok_or(EarlyEnd)?;
                walker = &walker[1..];
                x
            }}
        }

        fn i8_from_byte(b: u8) -> i8 {
            unsafe { mem::transmute(b) }
        }

        macro_rules! read_i1 {
            () => {{
                i8_from_byte(read_u1!())
            }}
        }

        macro_rules! read_u2 {
            () => {{
                let x = u16::from_be_bytes(walker[0..2].try_into().map_err(|_| EarlyEnd)?);
                walker = &walker[2..];
                x
            }}
        }

        fn i16_from_short(b: u16) -> i16 {
            unsafe { mem::transmute(b) }
        }

        macro_rules! read_i2 {
            () => {{
                i16_from_short(read_u2!())
            }};
        }

        let mut pc = 0;

        macro_rules! index_from_relative {
            ($relative:expr) => {{
                let raw = $relative as i32;
                let current_byte_offset = *instruction_index_table
                    .get(&pc).ok_or(InvalidCode)? as i32;
                let byte_index = (raw + current_byte_offset) as u32;
                *byte_offset_table
                    .get(&byte_index)
                    .ok_or(InvalidCode)?
            }}
        }

        while walker.len() > 0 {

            let opcode = read_u1!();

            code.push(match opcode {
                0x00 => Nop,
                0x01 => AconstNull,
                0x02 => Ipush(-1),
                0x03 => Ipush(0),
                0x04 => Ipush(1),
                0x05 => Ipush(2),
                0x06 => Ipush(3),
                0x07 => Ipush(4),
                0x08 => Ipush(5),
                0x10 => Ipush(read_i1!() as i16),
                0x11 => Ipush(read_i2!()),
                0x1a => Iload(0),
                0x1b => Iload(1),
                0x2a => Aload(0),
                0x3b => Istore(0),
                0x3c => Istore(1),
                0x3d => Istore(2),
                0x3e => Istore(3),
                0x68 => Imul,
                0x84 => Iinc(read_u1!(), read_i1!()),
                0x9f => IfIcmpeq(index_from_relative!(read_i2!())),
                0xa0 => IfIcmpne(index_from_relative!(read_i2!())),
                0xa1 => IfIcmplt(index_from_relative!(read_i2!())),
                0xa2 => IfIcmpge(index_from_relative!(read_i2!())),
                0xa3 => IfIcmpgt(index_from_relative!(read_i2!())),
                0xa4 => IfIcmple(index_from_relative!(read_i2!())),
                0xa7 => Goto(index_from_relative!(read_i2!())),
                0xac => Ireturn,
                0xb1 => Return,
                0xb7 => Invokespecial(read_u2!()),
                _ => return Err(InvalidOpcode(opcode))
            });

            pc += 1;
        }

        Ok(code)
    }
}

#[cfg(test)]
mod tests {
    use std::mem;
    use crate::bytecode::Instruction;
    use crate::bytecode::Instruction::GotoW;

    #[test]
    fn instruction_size() {
        assert!(8 >= mem::size_of::<Instruction>());
    }
}