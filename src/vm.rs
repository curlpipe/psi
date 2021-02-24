// vm.rs - Stack-based Bytecode Virtual Machine
use crate::{Chunk, Error, OpCode, Value};

const STACK_SIZE: usize = 256;

#[allow(clippy::upper_case_acronyms)]
pub struct VM {
    chunk: Chunk,
    stack: Vec<Value>,
    verbose: bool,
}

impl VM {
    pub fn new(verbose: bool) -> Self {
        // Create a new virtual machine
        Self {
            chunk: Chunk::new(0),
            stack: Vec::with_capacity(STACK_SIZE),
            verbose,
        }
    }

    pub fn run(&mut self, chunk: Chunk) -> Result<(), Error> {
        // Execute a bytecode chunk
        self.chunk = chunk;
        for instruction in self.chunk.code.clone() {
            // Display stack if verbose option specified
            if self.verbose {
                for slot in &self.stack {
                    print!("[ {} ]", slot);
                }
                println!();
            }
            // Carry out instruction
            match instruction {
                OpCode::OpConstant(idx) => {
                    let constant = self.chunk.constants[idx as usize].clone();
                    self.stack.push(constant);
                }
                OpCode::OpNegate => {
                    let operand = -self.stack.pop().unwrap();
                    self.stack.push(operand);
                }
                OpCode::OpAdd => self.bin_op("+"),
                OpCode::OpSub => self.bin_op("-"),
                OpCode::OpMul => self.bin_op("*"),
                OpCode::OpDiv => self.bin_op("/"),
                OpCode::OpReturn => {
                    println!("{}", self.stack.pop().unwrap());
                    break
                }
            }
        }
        Ok(())
    }

    fn bin_op(&mut self, op: &str) {
        // Execute a binary operation
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        match op {
            "+" => self.stack.push(a + b),
            "-" => self.stack.push(a - b),
            "*" => self.stack.push(a * b),
            "/" => self.stack.push(a / b),
            _ => unreachable!(),
        }
    }
}

