// compiler.rs - For emitting bytecode given a stream of tokens
use crate::{Chunk, Token, OpCode, Error, TokenKind, Value, Precedence, get_rule};

pub struct Compiler {
    tokens: Vec<Token>,
    pub chunk: Chunk,
    ptr: usize,
}

impl Compiler {
    pub fn new(tokens: Vec<Token>) -> Self {
        // Create a new compiler
        Self {
            tokens,
            chunk: Chunk::new(1),
            ptr: 0,
        }
    }

    pub fn compile(&mut self) -> Result<(), Error> {
        // Start the compilation
        self.expression()?;
        self.consume(TokenKind::EOI)?;
        self.end_compiler();
        Ok(())
    }

    fn expression(&mut self) -> Result<(), Error> {
        // Compile an expression, starting with the highest precedence
        self.parse_precedence(Precedence::Assignment)?;
        Ok(())
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<(), Error> {
        // Parse a precedence level
        let precedence = precedence as u8;
        self.advance();
        // Get the left hand side
        let current = self.get_back().unwrap();
        // Handle a prefix rule (allows for negation)
        let prefix_rule = get_rule(current.kind).prefix;
        if let Some(prefix) = prefix_rule {
            prefix(self)?;
            // Walk down the precedence
            while precedence <= get_rule(self.get().unwrap().kind).prec as u8 {
                self.advance();
                // Handle an infix rule (allows for arithmetic operations)
                let infix_rule = get_rule(self.get_back().unwrap().kind).infix;
                if let Some(infix) = infix_rule {
                    infix(self)?;
                }
            }
            Ok(())
        } else {
            // There was no expression here
            Err(Error::ExpectedExpression(current.line, current.col))
        }
    }

    pub fn binary(&mut self) -> Result<(), Error> {
        // Compile a binary operation
        let op_type = self.get_back().unwrap();
        let rule = get_rule(op_type.kind);
        // Move onto the lower precedence
        self.parse_precedence(rule.prec.shift())?;
        // Emit the correct operation
        self.emit_byte(match op_type.kind {
            TokenKind::Plus => OpCode::OpAdd,
            TokenKind::Minus => OpCode::OpSub,
            TokenKind::Asterisk => OpCode::OpMul,
            TokenKind::Slash => OpCode::OpDiv,
            TokenKind::Percent => OpCode::OpMod,
            TokenKind::Hat => OpCode::OpPow,
            _ => unreachable!(),
        });
        Ok(())
    }

    pub fn number(&mut self) -> Result<(), Error> {
        // Emit a number constant
        self.emit_constant(match self.get_back().unwrap().kind {
            TokenKind::Number(float) => Value::Number(float),
            TokenKind::Nil => Value::Nil,
            TokenKind::True => Value::Boolean(true),
            TokenKind::False => Value::Boolean(false),
            _ => unreachable!(),
        });
        Ok(())
    }

    pub fn grouping(&mut self) -> Result<(), Error> {
        // Compile a grouping operation, this is for brackets
        self.expression()?;
        self.consume(TokenKind::RightParen)?;
        Ok(())
    }

    pub fn unary(&mut self) -> Result<(), Error> {
        // Compile a unary operation like negation or not
        let op_type = self.get_back().unwrap();
        self.parse_precedence(Precedence::Unary)?;
        match op_type.kind {
            TokenKind::Minus => self.emit_byte(OpCode::OpNegate),
            _ => unreachable!(),
        }
        Ok(())
    }

    fn end_compiler(&mut self) {
        // Finalise compilation
        self.emit_return();
    }

    fn emit_byte(&mut self, code: OpCode) {
        // Emit a byte into the chunk
        self.chunk.write(code);
    }

    fn emit_constant(&mut self, val: Value) {
        // Create and emit a new constant
        let idx = self.chunk.add_constant(val);
        self.emit_byte(OpCode::OpConstant(idx))
    }

    fn emit_return(&mut self) {
        // Emit a return operation
        self.emit_byte(OpCode::OpReturn)
    }

    fn advance(&mut self) {
        // Move the token focus forward
        self.ptr += 1;
    }

    fn consume(&mut self, kind: TokenKind) -> Result<(), Error> {
        // Consume a token if present, otherwise display an error
        let current = self.get().ok_or(Error::UnexpectedEOI)?;
        if current.kind == kind {
            self.advance();
            Ok(())
        } else {
            Err(Error::ExpectedToken(kind, current.line, current.col))
        }
    }

    fn get(&self) -> Option<Token> {
        // Retrieve the current token focus
        Some(*self.tokens.get(self.ptr)?)
    }

    fn get_back(&self) -> Option<Token> {
        // Look back at the token before the focus
        Some(*self.tokens.get(self.ptr - 1)?)
    }

    pub fn display(&self) {
        // Display the compiled chunk
        self.chunk.display();
    }
}
