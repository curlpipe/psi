// error.rs - For handling the formatting and types of errors
use thiserror::Error as ThisError;
use crate::TokenKind;

#[derive(ThisError, Debug)]
pub enum Error {
    // When the lexer hits a character it doesn't know, e.g. a unicode char
    #[error("Error: [line {1} column {2}] Unexpected character: '{0}'")]
    UnexpectedCharacter(char, usize, usize),
    // When the lexer hits the EOI while collecting a token e.g. unterminated string
    #[error("Error: Unexpected end of input")]
    UnexpectedEOI,
    // When the consume method misses a token e.g. missing end bracket
    #[error("Error: [line {1} column {2}] Expected {0}")]
    ExpectedToken(TokenKind, usize, usize),
    // When the the compiler tries to parse a dodgy token stream e.g. "1 + * 2"
    #[error("Error: [line {0} column {1}] Expected expression")]
    ExpectedExpression(usize, usize),
    // When the user provides incorrect types in operations e.g. "-false + true"
    #[error("Error: [line {0} column {1}] Mismatched types: {2}")]
    MismatchedTypes(usize, usize, String),
}

