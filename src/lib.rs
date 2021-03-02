// Psibyte - A bytecode implementation of the PSI language
pub mod precedence;
pub mod compiler;
pub mod chunk;
pub mod error;
pub mod lexer;
pub mod value;
pub mod vm;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub use precedence::{Precedence, get_rule};
pub use lexer::{Token, Lexer, TokenKind};
pub use chunk::{OpCode, Chunk};
pub use compiler::Compiler;
pub use error::Error;
pub use value::Value;
pub use vm::VM;
