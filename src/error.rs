// error.rs - For handling the formatting and types of errors
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr as width;
use thiserror::Error as ThisError;
use lliw::{Fg, Style, Reset};
use crate::TokenKind;

#[derive(ThisError, Debug)]
pub enum Error {
    // When the lexer hits a character it doesn't know, e.g. a unicode char
    #[error("[line {1}:{2}] Unexpected character: '{0}'")]
    UnexpectedCharacter(char, usize, usize, usize),
    // When the lexer hits the EOI while collecting a token e.g. unterminated string
    #[error("Unexpected end of input")]
    UnexpectedEOI,
    // When the consume method misses a token e.g. missing end bracket
    #[error("[line {1}:{2}] Expected {0}")]
    ExpectedToken(TokenKind, usize, usize, usize),
    // When the the compiler tries to parse a dodgy token stream e.g. "1 + * 2"
    #[error("[line {0}:{1}] Expected expression")]
    ExpectedExpression(usize, usize, usize),
    // When the user provides incorrect types in operations e.g. "-false + true"
    #[error("[line {0}:{1}] Mismatched types: {3}")]
    MismatchedTypes(usize, usize, usize, String),
}

impl Error {
    pub fn display_line(&self, line: &str) {
        let (col, len) = match self {
            Error::UnexpectedCharacter(_, _, c, l) => (*c, *l),
            Error::UnexpectedEOI => (width::width(line), 1),
            Error::ExpectedToken(_, _, c, l) => (*c, *l),
            Error::ExpectedExpression(_, c, l) => (*c, *l),
            Error::MismatchedTypes(_, c, l, _) => (*c, *l),
        };
        let mut line: Vec<&str> = line.graphemes(true).collect();
        let before: &str = &line[0..col - 1].join("");
        let (during, after);
        if col > line.len() {
            line.push(" ");
            during = line[col - 1..col + len].join("");
            after = "".to_string();
        } else {
            during = line[col - 1..col + len - 1].join("");
            after = line[col + len - 1..].join("");
        };
        println!(
            "  {}{}{}{}{}{}{}{}{}{}", 
            Style::Bold,
            Fg::Green, before, Fg::Red,
            Style::Underline, 
            during, 
            Style::NoUnderline,
            Fg::Green, after, Reset
        );
    }
}
