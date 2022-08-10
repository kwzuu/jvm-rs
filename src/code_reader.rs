use crate::attribute_info::AttributeInfo;
use crate::bytecode::{BytecodeParseError, Instruction};
use crate::code::{Code, ExceptionTableItem};
use crate::code_reader::CodeParseError::{EarlyEnd, InvalidFormat};
use std::slice::Iter;

pub struct CodeReader<'a> {
    bytes: Iter<'a, u8>,
}

#[derive(Debug, Clone)]
pub enum CodeParseError<'a> {
    EarlyEnd(&'a str),
    InvalidFormat,
}

impl<'a> CodeReader<'a> {
    pub fn new(bytes: &'a Vec<u8>) -> CodeReader<'a> {
        CodeReader {
            bytes: bytes.iter(),
        }
    }

    fn read_u1(&mut self) -> Option<u8> {
        self.bytes.next().map(|x| *x)
    }

    fn read_u2(&mut self) -> Option<u16> {
        let hi = self.read_u1()? as u16;
        let lo = self.read_u1()? as u16;
        Some(hi << 8 | lo)
    }

    fn read_u4(&mut self) -> Option<u32> {
        let hi = self.read_u1()? as u32;
        let lo = self.read_u1()? as u32;
        Some(hi << 16 | lo)
    }

    fn read_attribute(&mut self) -> Result<AttributeInfo, CodeParseError<'a>> {
        let mut ai = AttributeInfo {
            name_index: self.read_u2().ok_or(EarlyEnd("attr name index"))?,
            attribute_length: self.read_u4().ok_or(EarlyEnd("attr len"))?,
            info: vec![],
        };
        ai.info.reserve(ai.attribute_length as usize);
        for _ in 0..ai.attribute_length {
            match self.bytes.next() {
                Some(x) => ai.info.push(*x),
                None => return Err(EarlyEnd("attr")),
            }
        }
        Ok(ai)
    }

    fn read_exception_table_item(&mut self) -> Option<ExceptionTableItem> {
        Some(ExceptionTableItem {
            start_pc: self.read_u2()?,
            end_pc: self.read_u2()?,
            catch_type: self.read_u2()?,
            handler_pc: self.read_u2()?,
        })
    }

    pub fn read_code(&mut self) -> Result<Code, CodeParseError<'a>> {
        let mut code = Code {
            max_stack: self.read_u2().ok_or(EarlyEnd("max stack"))?,
            max_locals: self.read_u2().ok_or(EarlyEnd("max locals"))?,
            code: vec![],
            exception_table: vec![],
            attributes: vec![],
        };

        let code_length = self.read_u4().ok_or(EarlyEnd("code len"))?;

        let mut bytecode: Vec<u8> = Vec::with_capacity(code_length as usize);
        for _ in 0..code_length {
            bytecode.push(self.read_u1().ok_or(EarlyEnd("bytecode"))?)
        }
        let mut bytecode = bytecode.iter();

        loop {
            code.code.push(match Instruction::read_from(&mut bytecode) {
                Ok(x) => x,
                Err(BytecodeParseError::EarlyEnd) => break,
                Err(BytecodeParseError::InvalidOpcode(_)) => return Err(InvalidFormat),
            });
        }

        let exception_table_length = self.read_u2().unwrap();
        code.exception_table
            .reserve(exception_table_length as usize);

        for _ in 0..exception_table_length {
            code.exception_table.push(
                self.read_exception_table_item()
                    .ok_or(EarlyEnd("exception table"))?,
            );
        }

        let attributes_count = self.read_u2().unwrap();
        code.attributes.reserve(attributes_count as usize);

        for _ in 0..attributes_count {
            code.attributes.push(self.read_attribute()?);
        }

        Ok(code)
    }
}
