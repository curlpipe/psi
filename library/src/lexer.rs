// lexer.rs - For turning streams of characters into tokens
use crate::Error;
use std::fmt;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Single character tokens
    Plus, Minus, Asterisk, Slash, Percent, Hat,
    LeftParen, RightParen, Exclamation,
    // Double character tokens
    Equals, NotEquals,
    Greater, Less, GreaterEq, LessEq,
    // Datatypes
    Number(f64), String(String),
    // Keywords
    True, False, Nil, Not,
    // Special
    EOI, Comment,
}

impl fmt::Display for TokenKind {
    #[cfg(not(tarpaulin_include))]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        // Define how to display token kinds
        match self {
            Self::Number(_) => write!(fmt, "number"),
            Self::String(_) => write!(fmt, "string"),
            Self::Comment => write!(fmt, "comment"),
            Self::Plus => write!(fmt, "'+'"),
            Self::Minus => write!(fmt, "'-'"),
            Self::Asterisk => write!(fmt, "'*'"),
            Self::Slash => write!(fmt, "'/'"),
            Self::Percent => write!(fmt, "'%'"),
            Self::Hat => write!(fmt, "'^'"),
            Self::LeftParen => write!(fmt, "'('"),
            Self::RightParen => write!(fmt, "')'"),
            Self::Exclamation => write!(fmt, "'!'"),
            Self::Greater => write!(fmt, "'>'"),
            Self::Less => write!(fmt, "'<'"),
            Self::GreaterEq => write!(fmt, "'>='"),
            Self::LessEq => write!(fmt, "'<='"),
            Self::Equals => write!(fmt, "'=='"),
            Self::NotEquals => write!(fmt, "'!='"),
            Self::True => write!(fmt, "'true'"),
            Self::False => write!(fmt, "'false'"),
            Self::Nil => write!(fmt, "'nil'"),
            Self::Not => write!(fmt, "'not'"),
            Self::EOI => write!(fmt, "end of input"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub len: usize,
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
            let (ptr, line, col) = (self.ptr, self.line, self.col);
            match c {
                // Capture single character tokens
                '+' => self.mk_token(TokenKind::Plus, 1),
                '-' => self.mk_token(TokenKind::Minus, 1),
                '*' => self.mk_token(TokenKind::Asterisk, 1),
                '%' => self.mk_token(TokenKind::Percent, 1),
                '^' => self.mk_token(TokenKind::Hat, 1),
                '(' => self.mk_token(TokenKind::LeftParen, 1),
                ')' => self.mk_token(TokenKind::RightParen, 1),
                // Check for the == token
                '=' => if self.peek(1) == Some('=') {
                    self.advance();
                    self.mk_long_token(TokenKind::Equals, [2, ptr, line, col])
                }
                // Check for the > or >= tokens
                '>' => if self.peek(1) == Some('=') {
                    self.advance();
                    self.mk_long_token(TokenKind::GreaterEq, [2, ptr, line, col]);
                } else {
                    self.mk_token(TokenKind::Greater, 1)
                }
                // Check for the < or <= tokens
                '<' => if self.peek(1) == Some('=') {
                    self.advance();
                    self.mk_long_token(TokenKind::LessEq, [2, ptr, line, col]);
                } else {
                    self.mk_token(TokenKind::Less, 1)
                }
                // Check for ! or != token
                '!' => if self.peek(1) == Some('=') { 
                    self.advance();
                    self.mk_long_token(TokenKind::NotEquals, [2, ptr, line, col])
                } else {
                    self.mk_token(TokenKind::Exclamation, 1)
                }
                // Check for a single line or multiline comment or / token
                '/' => if self.peek(1) == Some('/') {
                    // Single line comment
                    let mut len = 2;
                    self.advance();
                    self.advance();
                    while let Some(c) = self.get() {
                        // Keep on walkin' to the end of the line
                        if c == '\n' {
                            self.line += 1;
                            self.col = 1;
                            break
                        } else {
                            len += 1;
                            self.advance();
                        }
                    }
                    self.mk_long_token(TokenKind::Comment, [len, ptr, line, col]);
                } else if Some('*') == self.peek(1) {
                    // Mulitline comment
                    let mut len = 2;
                    self.advance();
                    self.advance();
                    while let Some(c) = self.get() {
                        // Keep on walkin' to the end of the comment
                        if c == '*' && self.peek(1) == Some('/') {
                            self.advance();
                            break;
                        } else if c == '\n' {
                            self.line += 1;
                            self.col = 1;
                            len += 1;
                            self.advance();
                        } else {
                            len += 1;
                            self.advance();
                        }
                    }
                    self.mk_long_token(TokenKind::Comment, [len + 2, ptr, line, col]);
                } else {
                    // Just your average slash character
                    self.mk_token(TokenKind::Slash, 1);
                }
                // Capture identifiers and keywords
                'a'..='z' => { 
                    self.word()?; 
                    continue; 
                }
                // Capture numbers
                '0'..='9' => { 
                    self.number();
                    continue;
                }
                // Capture strings
                '"' => { 
                    self.string()?;
                    continue;
                }
                // Ignore whitespace
                ' ' | '\t' => (),
                // Handle newline
                '\n' => {
                    self.line += 1;
                    self.col = 1;
                }
                // Handle the event of an unrecognised character
                _ => return Err(Error::UnexpectedCharacter(c, self.line, self.col, 1)),
            }
            self.advance();
        }
        // Append an EOI (end of input) token
        self.mk_token(TokenKind::EOI, 0);
        Ok(())
    }

    fn string(&mut self) -> Result<(), Error> {
        // Create a string token
        let (ptr, line, col) = (self.ptr, self.line, self.col);
        let mut result = String::new();
        self.advance();
        loop {
            // Run through string characters
            if let Some(c) = self.get() {
                // There are still characters
                if c == '"' && self.peek(-1) != Some('\\') {
                    // A quote (non-escaped)
                    self.advance();
                    break;
                } else if c == '\n' {
                    // A newline token (allows for newlines in strings)
                    self.line += 1;
                    self.col = 1;
                }
                // Push the character into the strings content
                self.advance();
                result.push(c);
            } else {
                // Unterminated string!
                return Err(Error::UnexpectedEOI("Unterminated string".to_string()));
            }
        }
        // Create string token
        let len = result.chars().count();
        self.mk_long_token(
            TokenKind::String(result), 
            [len + 2, ptr, line, col]
        );
        Ok(())
    }

    fn number(&mut self) {
        // Create a number token
        let (ptr, line, col) = (self.ptr, self.line, self.col);
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
            // Capture trailing numbers
            while let Some('0'..='9') = self.get() {
                result.push(self.get().unwrap());
                self.advance();
            }
        }
        // Create token and parse number
        self.mk_long_token(
            TokenKind::Number(result.parse().unwrap()), 
            [result.chars().count(), ptr, line, col]
        );
    }

    fn word(&mut self) -> Result<(), Error> {
        // For identifiers & keywords
        let (c, ptr, line, col) = (self.get().unwrap(), self.ptr, self.line, self.col);
        // Capture entire word
        let mut word = String::new();
        while let Some(c) = self.get() {
            if let 'a'..='z' = c {
                // Capture all the characters
                word.push(c);
                self.advance();
            } else {
                // Stop when you hit a non-alphabet character
                break;
            }
        }
        // Look up the word and determine if keyword or identifier
        match word.as_str() {
            "true" => self.mk_long_token(TokenKind::True, [4, ptr, line, col]),
            "false" => self.mk_long_token(TokenKind::False, [5, ptr, line, col]),
            "nil" => self.mk_long_token(TokenKind::Nil, [3, ptr, line, col]),
            "not" => self.mk_long_token(TokenKind::Not, [3, ptr, line, col]),
            //_ => return Err(Error::UnexpectedCharacter(c, line, col, width::width(word.as_str())))
            _ => return Err(Error::UnexpectedCharacter(c, line, col, 1))
        }
        Ok(())
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

    fn peek(&mut self, vec: isize) -> Option<char> {
        // Peek ahead a certain number of chars
        Some(*self.chars.get((self.ptr as isize + vec) as usize)?)
    }

    fn mk_long_token(&mut self, kind: TokenKind, start: [usize; 4]) {
        // Generates a long token with custom start points
        self.tokens.push(Token {
            kind,
            len: start[0],
            start: start[1],
            line: start[2],
            col: start[3],
        })
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
