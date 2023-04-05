use std::collections::HashMap;
use crate::attributes::attribute_info::AttributeInfo;
use crate::attributes::code::{Code, ExceptionTableItem};
use crate::attributes::code_reader::CodeParseError::{EarlyEnd, InvalidFormat};
use crate::bytecode::{BytecodeParseError, Instruction};
use std::slice::Iter;

pub struct CodeReader<'a> {
    bytes: Iter<'a, u8>,
}

#[derive(Debug, Clone)]
pub enum CodeParseError {
    EarlyEnd(String),
    InvalidFormat,
    BytecodeParseError(BytecodeParseError),
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
        let hi = self.read_u2()? as u32;
        let lo = self.read_u2()? as u32;
        Some(hi << 16 | lo)
    }

    fn read_attribute(&mut self) -> Result<AttributeInfo, CodeParseError> {
        let mut ai = AttributeInfo {
            name_index: self.read_u2().ok_or(EarlyEnd("attr name index".to_string()))?,
            attribute_length: self.read_u4().ok_or(EarlyEnd("attr len".to_string()))?,
            info: vec![],
        };
        ai.info.reserve(ai.attribute_length as usize);
        for _ in 0..ai.attribute_length {
            match self.bytes.next() {
                Some(x) => ai.info.push(*x),
                None => return Err(EarlyEnd("attr".to_string())),
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

    pub fn read_code(&mut self) -> Result<Code, CodeParseError> {
        let mut code = Code {
            max_stack: self.read_u2().ok_or(EarlyEnd("max stack".to_string()))?,
            max_locals: self.read_u2().ok_or(EarlyEnd("max locals".to_string()))?,
            code: vec![],
            exception_table: vec![],
            attributes: vec![],
        };

        let code_length = self.read_u4().ok_or(EarlyEnd("code len".to_string()))?;
        let mut bytecode: Vec<u8> = Vec::with_capacity(code_length as usize);

        for _ in 0..code_length {
            bytecode.push(self.read_u1().ok_or(EarlyEnd("bytecode".to_string()))?)
        }

        code.code = Instruction::read_from(bytecode.as_slice(), code_length)
            .map_err(CodeParseError::BytecodeParseError)?;

        dbg!(code.code.as_slice());

        let exception_table_length = self.read_u2().unwrap();

        code.exception_table
            .reserve(exception_table_length as usize);

        for _ in 0..exception_table_length {
            code.exception_table.push(
                self.read_exception_table_item()
                    .ok_or(EarlyEnd("exception table".to_string()))?,
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
