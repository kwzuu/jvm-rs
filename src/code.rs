use crate::attribute_info::AttributeInfo;
use crate::bytecode::Instruction;

#[derive(Debug, Clone)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<Instruction>,
    pub exception_table: Vec<ExceptionTableItem>,
    pub attributes: Vec<AttributeInfo>, // name:data
}

#[derive(Debug, Clone)]
pub struct ExceptionTableItem {
    pub start_pc: u16,
    pub end_pc: u16,
    pub catch_type: u16,
    pub handler_pc: u16,
}
