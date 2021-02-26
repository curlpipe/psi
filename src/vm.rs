// vm.rs - Stack-based Bytecode Virtual Machine
use crate::{Chunk, Error, OpCode, Value};

const STACK_SIZE: usize = 256;

#[allow(clippy::upper_case_acronyms)]
pub struct VM {
    pub stack: Vec<Value>,
    chunk: Chunk,
    verbose: bool,
}

impl VM {
    pub fn new(verbose: bool) -> Self {
        // Create a new virtual machine
        Self {
            stack: Vec::with_capacity(STACK_SIZE),
            chunk: Chunk::new(0),
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
                OpCode::OpNegate => if let Some(Value::Number(_)) = self.peek(0) {
                    let operand = -self.stack.pop().unwrap();
                    self.stack.push(operand);
                } else {
                    return Err(Error::MismatchedTypes(
                        self.chunk.line,
                        1000,
                        "Operand must be a number.".to_string()
                    ));
                }
                OpCode::OpAdd => self.bin_op("+")?,
                OpCode::OpSub => self.bin_op("-")?,
                OpCode::OpMul => self.bin_op("*")?,
                OpCode::OpDiv => self.bin_op("/")?,
                OpCode::OpMod => self.bin_op("%")?,
                OpCode::OpPow => self.bin_op("^")?,
                OpCode::OpReturn => {
                    println!("{}", self.stack.pop().unwrap());
                    break
                }
            }
        }
        Ok(())
    }

    fn bin_op(&mut self, op: &str) -> Result<(), Error> {
        // Execute a binary operation
        let (a, b) = (self.peek(0), self.peek(1));
        if let (Some(&Value::Number(_)), Some(&Value::Number(_))) = (a, b) {
            let b = self.stack.pop().unwrap();
            let a = self.stack.pop().unwrap();
            match op {
                "+" => self.stack.push(a + b),
                "-" => self.stack.push(a - b),
                "*" => self.stack.push(a * b),
                "/" => self.stack.push(a / b),
                "%" => self.stack.push(a % b),
                "^" => self.stack.push(a ^ b),
                _ => unreachable!(),
            }
            Ok(())
        } else {
            return Err(Error::MismatchedTypes(
                self.chunk.line, 
                1000, 
                "Operands must be numbers.".to_string()
            ))
        }
    }

    fn peek(&self, distance: usize) -> Option<&Value> {
        self.stack.get(self.stack.len() - 1 - distance)
    }
}

