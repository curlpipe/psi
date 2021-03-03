// precedence.rs - utilities for handling precedence within the language
use crate::{Compiler, TokenKind, Error};

pub type CompilerFn<'a> = Option<fn(&'a mut Compiler) -> Result<(), Error>>;

pub struct ParseRule<'a> {
    pub prefix: CompilerFn<'a>,
    pub infix: CompilerFn<'a>,
    pub prec: Precedence,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Indices,
    Unary,
    Call,
    Primary,
}

impl Precedence {
    pub fn shift(self) -> Self {
        // Move down the precedence ladder
        match self {
            Self::None => Self::Assignment,
            Self::Assignment => Self::Or,
            Self::Or => Self::And,
            Self::And => Self::Equality,
            Self::Equality => Self::Comparison,
            Self::Comparison => Self::Term,
            Self::Term => Self::Factor,
            Self::Factor => Self::Indices,
            Self::Indices => Self::Unary,
            Self::Unary => Self::Call,
            Self::Call => Self::Primary,
            Self::Primary => Self::None,
        }
    }
}

pub fn get_rule<'a>(kind: TokenKind) -> ParseRule<'a> {
    // Take in a token and work out what rule to handle it with
    ParseRule {
        prefix: match kind {
            TokenKind::LeftParen => Some(Compiler::grouping),
            TokenKind::Minus => Some(Compiler::unary),
            TokenKind::False => Some(Compiler::literal),
            TokenKind::True => Some(Compiler::literal),
            TokenKind::Nil => Some(Compiler::literal),
            TokenKind::Exclamation => Some(Compiler::unary),
            TokenKind::Not => Some(Compiler::unary),
            TokenKind::Number(_) => Some(Compiler::number),
            TokenKind::String(_) => Some(Compiler::string),
            _ => None,
        },
        infix: match kind {
            TokenKind::Minus => Some(Compiler::binary),
            TokenKind::Plus => Some(Compiler::binary),
            TokenKind::Asterisk => Some(Compiler::binary),
            TokenKind::Slash => Some(Compiler::binary),
            TokenKind::Percent => Some(Compiler::binary),
            TokenKind::Hat => Some(Compiler::binary),
            TokenKind::Equals => Some(Compiler::binary),
            TokenKind::NotEquals => Some(Compiler::binary),
            TokenKind::Greater => Some(Compiler::binary),
            TokenKind::GreaterEq => Some(Compiler::binary),
            TokenKind::Less => Some(Compiler::binary),
            TokenKind::LessEq => Some(Compiler::binary),
            _ => None,
        },
        prec: match kind {
            TokenKind::Minus => Precedence::Term,
            TokenKind::Plus => Precedence::Term,
            TokenKind::Slash => Precedence::Factor,
            TokenKind::Asterisk => Precedence::Factor,
            TokenKind::Percent => Precedence::Factor,
            TokenKind::Hat => Precedence::Indices,
            TokenKind::Equals => Precedence::Equality,
            TokenKind::NotEquals => Precedence::Equality,
            TokenKind::Greater => Precedence::Comparison,
            TokenKind::GreaterEq => Precedence::Comparison,
            TokenKind::Less => Precedence::Comparison,
            TokenKind::LessEq => Precedence::Comparison,
            _ => Precedence::None,
        },
    }
}

