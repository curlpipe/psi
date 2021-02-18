use crate::{Expr, Op};
use smartstring::alias::String;

pub type Identifier = Vec<String>;
pub type Identifiers = Vec<Expr>;
pub type Block = Vec<Action>;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Action {
    // Comments
    Comment(String),
    // Asignment
    VarAssign(Expr, Expr),
    SingleShot(Expr, Op, Expr),
    // Selection
    If(Vec<IfPart>),
    // Functions
    FnCall(Expr, Identifiers),
    FnDef(Expr, Identifier, Block),
    Return(Option<Expr>),
    // Iteration
    Loop(Block),
    Break,
    Continue,
    For(Identifier, Expr, Block),
    While(Expr, Block),
    // Importing
    Import(Expr, Option<Expr>),
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum IfPart {
    If(Expr, Block),
    IfElse(Expr, Block),
    Else(Block),
}

impl Action {
    pub fn to_expr(&self) -> Expr {
        match self {
            Action::FnCall(i, a) => Expr::FnCall(Box::new(i.to_owned()), a.to_owned()),
            _ => panic!("Impossible operation"),
        }
    }
}
