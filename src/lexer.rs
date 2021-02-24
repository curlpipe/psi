// lexer.rs - For turning streams of characters into tokens
use crate::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Number(f64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
    EOI,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number(_) => write!(fmt, "number"),
            Self::Plus => write!(fmt, "'+'"),
            Self::Minus => write!(fmt, "'-'"),
            Self::Asterisk => write!(fmt, "'*'"),
            Self::Slash => write!(fmt, "'/'"),
            Self::LeftParen => write!(fmt, "'('"),
            Self::RightParen => write!(fmt, "')'"),
            Self::EOI => write!(fmt, "end of input"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    start: usize,
    len: usize,
    pub line: usize,
    pub col: usize,
}

pub struct Lexer {
    chars: Vec<char>,
    pub tokens: Vec<Token>,
    ptr: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(src: &str) -> Self {
        // Create a new lexer
        Self {
            chars: src.chars().collect(),
            tokens: vec![],
            ptr: 0,
            line: 1,
            col: 1
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        // Run the lexer
        while let Some(c) = self.get() {
            match c {
                // Capture single character tokens
                '+' => self.mk_token(TokenKind::Plus, 1),
                '-' => self.mk_token(TokenKind::Minus, 1),
                '*' => self.mk_token(TokenKind::Asterisk, 1),
                '/' => self.mk_token(TokenKind::Slash, 1),
                '(' => self.mk_token(TokenKind::LeftParen, 1),
                ')' => self.mk_token(TokenKind::RightParen, 1),
                // Capture numbers
                '0'..='9' => { 
                    self.number();
                    continue;
                }
                // Ignore whitespace
                ' ' | '\t' => (),
                // Handle newline
                '\n' => {
                    self.line += 1;
                    self.col = 1;
                }
                _ => return Err(Error::UnexpectedCharacter(c, self.line, self.col)),
            }
            self.advance();
        }
        // Append an EOI (end of input) token
        self.mk_token(TokenKind::EOI, 0);
        Ok(())
    }

    fn number(&mut self) {
        // Create a number token
        let mut result = String::new();
        // Collect all digits
        while let Some('0'..='9') = self.get() {
            result.push(self.get().unwrap());
            self.advance();
        }
        // Allow for float
        if let Some('.') = self.get() {
            result.push('.');
            self.advance();
            while let Some('0'..='9') = self.get() {
                result.push(self.get().unwrap());
                self.advance();
            }
        }
        // Create token and parse number
        self.mk_token(
            TokenKind::Number(result.parse().unwrap()), 
            result.chars().count()
        );
    }

    fn advance(&mut self) {
        // To move the character focus forward
        self.ptr += 1;
        self.col += 1;
    }

    fn get(&mut self) -> Option<char> {
        // To get the current character focus
        Some(*self.chars.get(self.ptr)?)
    }

    fn mk_token(&mut self, kind: TokenKind, len: usize) {
        // Generates a token from the current status
        self.tokens.push(Token {
            kind,
            len,
            start: self.ptr,
            line: self.line,
            col: self.col,
        })
    }

    pub fn display(&self) {
        // Display the token stream
        print!("=> ");
        for i in &self.tokens {
            print!("[ {:?} ] ", i.kind);
        }
        println!()
    }
}
