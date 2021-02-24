// chunk.rs - Utilities for representing chunks of bytecode
use crate::Value;
use std::fmt;

#[derive(Clone)]
pub enum OpCode {
    OpConstant(u16),
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpMod,
    OpReturn,
    OpNegate,
}

pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    line: usize,
}

impl Chunk {
    pub fn new(line: usize) -> Self {
        // Create a new chunk
        Self {
            code: vec![],
            constants: Vec::with_capacity(256),
            line,
        }
    }

    pub fn write(&mut self, code: OpCode) {
        // Add an instruction to this chunk
        self.code.push(code)
    }

    pub fn add_constant(&mut self, value: Value) -> u16 {
        // Add a constant to this chunk
        let idx = self.constants.len();
        self.constants.push(value);
        idx as u16
    }

    pub fn display(&self) {
        // Display the chunk in text format
        for i in &self.code {
            self.disassemble_instruction(i);
        }
    }

    pub fn disassemble_instruction(&self, instruction: &OpCode) {
        // Disassemble and display an instruction
        match instruction {
            OpCode::OpConstant(idx) => println!(
                "=> {:04} {} {} {}", 
                self.line, 
                instruction, 
                idx, 
                self.constants[*idx as usize]
            ),
            _ => println!(
                "=> {:04} {}", 
                self.line, 
                instruction
            ),
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpCode::OpConstant(_) => write!(fmt, "OP_CONSTANT"),
            OpCode::OpAdd => write!(fmt, "OP_ADD"),
            OpCode::OpSub => write!(fmt, "OP_SUB"),
            OpCode::OpMul => write!(fmt, "OP_MUL"),
            OpCode::OpDiv => write!(fmt, "OP_DIV"),
            OpCode::OpMod => write!(fmt, "OP_MOD"),
            OpCode::OpReturn => write!(fmt, "OP_RETURN"),
            OpCode::OpNegate => write!(fmt, "OP_NEGATE"),
        }
    }
}

