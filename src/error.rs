// error.rs - For handling the formatting and types of errors
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr as width;
use thiserror::Error as ThisError;
use lliw::{Fg, Style, Reset};
use crate::TokenKind;

#[derive(ThisError, Debug, PartialEq)]
pub enum Error {
    // When the lexer hits a character it doesn't know, e.g. a unicode char
    #[error("[line {1}:{2}] Unexpected character: '{0}'")]
    UnexpectedCharacter(char, usize, usize, usize),
    // When the lexer hits the EOI while collecting a token e.g. unterminated string
    #[error("Unexpected end of input: {0}")]
    UnexpectedEOI(String),
    // When the consume method misses a token e.g. missing end bracket
    #[error("[line {1}:{2}] Expected {0}")]
    ExpectedToken(TokenKind, usize, usize, usize),
    // When the the compiler tries to parse a dodgy token stream e.g. "1 + * 2"
    #[error("[line {0}:{1}] Expected expression")]
    ExpectedExpression(usize, usize, usize),
    // When the user provides incorrect types in operations e.g. "-false + true"
    #[error("[line {0}:{1}] Mismatched types: {3}")]
    MismatchedTypes(usize, usize, usize, String),
    // When the user tries to do something impossible e.g. "hi" - "hello"
    #[error("[line {0}:{1}] Can't apply operation '{3}' to this type")]
    ImpossibleOperation(usize, usize, usize, String),
}

impl Error {
    pub fn display_line(&self, line: &str) {
        // This is a function that creates very nice error reporting info
        let (col, len) = match self {
            Error::UnexpectedCharacter(_, _, c, l) => (*c, *l),
            Error::UnexpectedEOI(_) => (width::width(line) + 1, 0),
            Error::ExpectedToken(_, _, c, l) => (*c, *l),
            Error::ExpectedExpression(_, c, l) => (*c, *l),
            Error::MismatchedTypes(_, c, l, _) => (*c, *l),
            Error::ImpossibleOperation(_, c, l, _) => (*c, *l),
        };
        // Split the source code into a list of strings
        let mut line: Vec<&str> = line.graphemes(true).collect();
        // Work out the part of the code before the problematic area
        let before: &str = &line[0..col - 1].join("");
        let (during, after);
        // Work out if we are reporting a column out of the span of the source
        if col > line.len() {
            // Insert a space to allow for reporting of invisible end tokens
            line.push(" ");
            // Work out the offending part and initiate an empty after part
            during = line[col - 1..col + len].join("");
            after = "".to_string();
        } else {
            // Grab the offending source code area and the part afterwards
            during = line[col - 1..col + len - 1].join("");
            after = line[col + len - 1..].join("");
        };
        // Format it and print it out
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
