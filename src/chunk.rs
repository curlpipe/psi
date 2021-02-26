// chunk.rs - Utilities for representing chunks of bytecode
use lliw::{Fg, Style, Reset};
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
    OpPow,
    OpReturn,
    OpNegate,
}

pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    pub line: usize,
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
                "=> {}{:04} {}{}{} {}{}{} {}{}", 
                Fg::Blue, self.line,
                Fg::LightBlack, Style::Bold, instruction, Style::NoBold,
                Fg::Blue, idx, self.constants[*idx as usize],
                Fg::Reset,
            ),
            _ => println!(
                "=> {}{:04} {}{}{}{}", 
                Fg::Blue, self.line, 
                Fg::LightBlack, Style::Bold, instruction,
                Reset,
            ),
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", match self {
            OpCode::OpConstant(_) => "OP_CONSTANT",
            OpCode::OpAdd => "OP_ADD",
            OpCode::OpSub => "OP_SUB",
            OpCode::OpMul => "OP_MUL",
            OpCode::OpDiv => "OP_DIV",
            OpCode::OpMod => "OP_MOD",
            OpCode::OpPow => "OP_POW",
            OpCode::OpReturn => "OP_RETURN",
            OpCode::OpNegate => "OP_NEGATE",
        })
    }
}

