// chunk.rs - Utilities for representing chunks of bytecode
use lliw::{Fg, Style, Reset};
use crate::Value;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    OpConstant(u16),
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpMod,
    OpPow,
    OpNegate,
    OpNot,
    OpTrue,
    OpFalse,
    OpNil,
    OpEqual,
    OpGreater,
    OpLess,
    OpReturn,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    pub code: Vec<(usize, usize, OpCode)>,
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

    pub fn write(&mut self, code: OpCode, col: usize, len: usize) {
        // Add an instruction to this chunk
        self.code.push((col, len, code))
    }

    pub fn add_constant(&mut self, value: Value) -> u16 {
        // Add a constant to this chunk
        let idx = self.constants.len();
        self.constants.push(value);
        idx as u16
    }

    pub fn display(&self) {
        // Display the chunk in text format
        for (c, _, i) in &self.code {
            self.disassemble_instruction(i, *c);
        }
    }

    pub fn disassemble_instruction(&self, instruction: &OpCode, col: usize) {
        // Disassemble and display an instruction
        match instruction {
            OpCode::OpConstant(idx) => println!(
                "=> {}{:04} {:03} {}{}{} {}{}{} {}{}", 
                Fg::Blue, self.line, col,
                Fg::LightBlack, Style::Bold, instruction, Style::NoBold,
                Fg::Blue, idx, self.constants[*idx as usize],
                Fg::Reset,
            ),
            _ => println!(
                "=> {}{:04} {:03} {}{}{}{}", 
                Fg::Blue, self.line, col,
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
            OpCode::OpTrue => "OP_TRUE",
            OpCode::OpFalse => "OP_FALSE",
            OpCode::OpNil => "OP_NIL",
            OpCode::OpReturn => "OP_RETURN",
            OpCode::OpNegate => "OP_NEGATE",
            OpCode::OpNot => "OP_NOT",
            OpCode::OpGreater => "OP_GREATER",
            OpCode::OpLess => "OP_LESS",
            OpCode::OpEqual => "OP_EQUAL",
        })
    }
}

