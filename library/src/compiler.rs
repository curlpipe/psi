// compiler.rs - For emitting bytecode given a stream of tokens
use crate::{
    Chunk, 
    Token, 
    OpCode, 
    Error, 
    TokenKind as Tk, 
    Value, 
    Precedence, 
    get_rule, 
    TokenKind
};

pub struct Compiler {
    tokens: Vec<Token>,
    pub chunk: Chunk,
    ptr: usize,
}

impl Compiler {
    pub fn new(tokens: Vec<Token>) -> Self {
        // Create a new compiler
        let mut ptr = 0;
        // If the token stream begins with comments, jump over them
        while let Some(t) = tokens.get(ptr) {
            if t.kind == TokenKind::Comment { ptr += 1; } else { break }
        }
        Self {
            tokens,
            chunk: Chunk::new(1),
            ptr,
        }
    }

    pub fn compile(&mut self) -> Result<(), Error> {
        // Start the compilation
        while let Ok(0) = self.present(TokenKind::EOI) {
            // Move until EOI is hit
            self.declaration()?;
        }
        Ok(())
    }

    fn declaration(&mut self) -> Result<(), Error> {
        if self.present(TokenKind::Var)? == 0 {
            self.statement()
        } else {
            self.var_declaration()
        }
    }

    fn statement(&mut self) -> Result<(), Error> {
        // Check for print statement
        let col = self.present(TokenKind::Print)?;
        if col != 0 {
            self.print_statement(col)?;
        } else {
            self.expression_statement()?;
        }
        Ok(())
    }

    fn expression(&mut self) -> Result<(), Error> {
        // Compile an expression, starting with the highest precedence
        self.parse_precedence(Precedence::Assignment)?;
        Ok(())
    }

    fn expression_statement(&mut self) -> Result<(), Error> {
        // Expression that acts as a statement
        self.expression()?;
        self.consume(TokenKind::Semicolon)?;
        self.emit_byte(OpCode::OpPop, self.get_back().unwrap().col, 0);
        Ok(())
    }

    fn var_declaration(&mut self) -> Result<(), Error> {
        // For variable declaration
        let global = self.parse_variable()?;
        let present = self.present(TokenKind::Equal)?;
        if present == 0 {
            self.emit_byte(OpCode::OpNil, present, 3);
        } else {
            self.expression()?;
        }
        self.consume(TokenKind::Semicolon)?;
        self.define_variable(global)?;
        Ok(())
    }

    fn print_statement(&mut self, col: usize) -> Result<(), Error> {
        // Consume a print statement and emit print operation
        self.expression()?;
        self.consume(TokenKind::Semicolon)?;
        self.emit_byte(OpCode::OpPrint, col, 5);
        Ok(())
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<(), Error> {
        // Parse a precedence level
        let precedence = precedence as u8;
        self.advance()?;
        // Get the left hand side
        let current = self.get_back().unwrap();
        // Handle a prefix rule (allows for negation)
        let prefix_rule = get_rule(current.kind).prefix;
        if let Some(prefix) = prefix_rule {
            // Work out if it is possible to assign to an expression
            let can_assign = precedence <= Precedence::Assignment as u8;
            prefix(self, can_assign)?;
            // Walk down the precedence
            while precedence <= get_rule(self.get().unwrap().kind).prec as u8 {
                self.advance()?;
                // Handle an infix rule (allows for arithmetic operations)
                let infix_rule = get_rule(self.get_back().unwrap().kind).infix;
                if let Some(infix) = infix_rule {
                    infix(self, can_assign)?;
                }
            }
            if !can_assign && self.present(TokenKind::Equal)? == 0 {
                Err(Error::InvalidAssignmentTarget(current.line, current.col, current.len))
            } else {
                Ok(())
            }
        } else {
            // There was no expression here
            Err(Error::ExpectedExpression(current.line, current.col, current.len))
        }
    }

    pub fn identifier_constant(&mut self, token: Token) -> Result<u16, Error> {
        if let Token { kind: TokenKind::Identifier(id), .. } = token {
            Ok(self.chunk.add_constant(Value::String(id)))
        } else {
            unreachable!()
        }
    }

    pub fn parse_variable(&mut self) -> Result<u16, Error> {
        let tok = self.get().unwrap();
        if let Token { kind: TokenKind::Identifier(_), .. } = tok {
            self.advance()?;
            self.identifier_constant(tok)
        } else {
            self.consume(TokenKind::Identifier("".to_string()))?;
            unreachable!()
        }
    }

    pub fn define_variable(&mut self, global: u16) -> Result<(), Error> {
        let semi = self.get_back().unwrap();
        self.emit_byte(OpCode::OpDefineGlobal(global), semi.col, semi.len);
        Ok(())
    }

    pub fn binary(&mut self, _: bool) -> Result<(), Error> {
        // Compile a binary operation
        let op_type = self.get_back().unwrap();
        let rule = get_rule(op_type.kind.clone());
        // Move onto the lower precedence
        self.parse_precedence(rule.prec.shift())?;
        // Emit the correct operation
        self.emit_byte(match op_type.kind {
            Tk::Plus => OpCode::OpAdd,
            Tk::Minus => OpCode::OpSub,
            Tk::Asterisk => OpCode::OpMul,
            Tk::Slash => OpCode::OpDiv,
            Tk::Percent => OpCode::OpMod,
            Tk::Hat => OpCode::OpPow,
            Tk::Equals => OpCode::OpEqual,
            Tk::Greater => OpCode::OpGreater,
            Tk::Less => OpCode::OpLess,
            Tk::NotEquals => OpCode::OpEqual,
            Tk::GreaterEq => OpCode::OpLess,
            Tk::LessEq => OpCode::OpGreater,
            _ => unreachable!(),
        }, op_type.col, op_type.len);
        // Inverse specific operations (more efficent than direct operations)
        if let Tk::NotEquals | Tk::GreaterEq | Tk::LessEq = op_type.kind {
            self.emit_byte(OpCode::OpNot, op_type.col, op_type.len);
        }
        Ok(())
    }

    pub fn literal(&mut self, _: bool) -> Result<(), Error> {
        // Emit a literal
        let val = self.get_back().unwrap();
        self.emit_byte(match val.kind {
            Tk::False => OpCode::OpFalse,
            Tk::True => OpCode::OpTrue,
            Tk::Nil => OpCode::OpNil,
            _ => unreachable!(),
        }, val.col, val.len);
        Ok(())
    }

    pub fn number(&mut self, _: bool) -> Result<(), Error> {
        // Emit a number constant
        let val = self.get_back().unwrap();
        if let Tk::Number(float) = val.kind {
            self.emit_constant(Value::Number(float), val.col, val.len);
        }
        Ok(())
    }

    pub fn string(&mut self, _: bool) -> Result<(), Error> {
        // Emit a string constant
        if let Some(Token{ kind: Tk::String(s), col, len, .. }) = self.get_back() {
            self.emit_constant(Value::String(s), col, len);
        }
        Ok(())
    }

    pub fn named_variable(&mut self, name: Token, can_assign: bool) -> Result<(), Error> {
        let arg = self.identifier_constant(name.clone())?;
        if self.present(TokenKind::Equal)? == 0 {
            self.emit_byte(OpCode::OpGetGlobal(arg), name.col, name.len);
        } else if can_assign {
            self.expression()?;
            self.emit_byte(OpCode::OpSetGlobal(arg), name.col, name.len);
        }
        Ok(())
    }

    pub fn variable(&mut self, can_assign: bool) -> Result<(), Error> {
        self.named_variable(self.get_back().unwrap(), can_assign)
    }

    pub fn grouping(&mut self, _: bool) -> Result<(), Error> {
        // Compile a grouping operation, this is for brackets
        self.expression()?;
        self.consume(Tk::RightParen)?;
        Ok(())
    }

    pub fn unary(&mut self, _: bool) -> Result<(), Error> {
        // Compile a unary operation like negation or not
        let op_type = self.get_back().unwrap();
        self.parse_precedence(Precedence::Unary)?;
        match op_type.kind {
            Tk::Minus => self.emit_byte(OpCode::OpNegate, op_type.col, op_type.len),
            Tk::Exclamation | Tk::Not => 
                self.emit_byte(OpCode::OpNot, op_type.col, op_type.len),
            _ => unreachable!(),
        }
        Ok(())
    }

    fn end_compiler(&mut self, end: usize) {
        // Finalise compilation
        self.emit_return(end, 0);
    }

    fn emit_byte(&mut self, code: OpCode, col: usize, len: usize) {
        // Emit a byte into the chunk
        self.chunk.write(code, col, len);
    }

    fn emit_constant(&mut self, val: Value, col: usize, len: usize) {
        // Create and emit a new constant
        let idx = self.chunk.add_constant(val);
        self.emit_byte(OpCode::OpConstant(idx), col, len)
    }

    fn emit_return(&mut self, col: usize, len: usize) {
        // Emit a return operation
        self.emit_byte(OpCode::OpReturn, col, len)
    }

    fn advance(&mut self) -> Result<(), Error> {
        // Move the token focus forward
        self.ptr += 1;
        while let Some(com) = self.get() {
            if com.kind != TokenKind::Comment { break }
            self.ptr += 1;
        }
        Ok(())
    }

    fn present(&mut self, kind: TokenKind) -> Result<usize, Error> {
        // Returns Ok(0) if not present, returns Ok(col) if present
        let tok = self.get().unwrap();
        if tok.kind != kind { Ok(0) }
        else { self.advance()?; Ok(tok.col) }
    }

    fn consume(&mut self, kind: Tk) -> Result<usize, Error> {
        // Consume a token if present, otherwise display an error
        let current = self.get().ok_or_else(|| Error::UnexpectedEOI(format!("Expected {}", kind)))?;
        if current.kind == kind {
            self.advance().unwrap();
            Ok(current.col)
        } else if current.kind == TokenKind::EOI && kind != TokenKind::EOI {
            // Run back along the line to find appropriate place for token
            if self.tokens[self.ptr - 1].kind == TokenKind::Comment {
                self.ptr -= 1;
                while let Some(Token { kind: TokenKind::Comment, .. }) = self.get_back() {
                    self.ptr -= 1;
                }
            }
            let current = self.get().unwrap();
            let len = if current.kind == TokenKind::EOI { 0 } else { 1 };
            Err(Error::ExpectedToken(kind, current.line, current.col, len))
        } else {
            Err(Error::ExpectedToken(kind, current.line, current.col, current.len))
        }
    }

    fn get(&self) -> Option<Token> {
        // Retrieve the current token focus
        Some(self.tokens.get(self.ptr)?.clone())
    }

    fn get_back(&self) -> Option<Token> {
        // Look back at the token before the focus
        let mut target = self.tokens.get(self.ptr - 1)?;
        let mut offset: isize = -1;
        while let TokenKind::Comment = target.kind {
            offset -= 1;
            if self.ptr as isize + offset < 0 { return None; }
            target = self.tokens.get((self.ptr as isize + offset) as usize)?;
        }
        Some(target.clone())
    }

    pub fn display(&self) {
        // Display the compiled chunk
        self.chunk.display();
    }
}
