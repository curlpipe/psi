use crate::{Error, Identifier, Identifiers, Table};
use smartstring::alias::String;
use std::collections::VecDeque;
use std::fmt;
use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    StringFmt(Identifiers),
    StringRaw(String),
    Identifier(Identifier),
    Boolean(bool),
    Inclusive(Box<Expr>, Box<Expr>),
    Exclusive(Box<Expr>, Box<Expr>),
    Array(Identifiers),
    Table(Table),
    Not(Box<Expr>),
    BinOp(Box<Expr>, Op, Box<Expr>),
    FnCall(Box<Expr>, Identifiers),
    VarIndex(Box<Expr>, VecDeque<Expr>),
    Impossible,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Rem,
    And,
    Or,
    In,
    Greater,
    Less,
    GreaterEq,
    LessEq,
    Equals,
    NotEquals,
}

impl Add for Expr {
    type Output = Self;
    fn add(self, by: Self) -> Self::Output {
        match (self, by) {
            (Self::Float(f1), Self::Float(f2)) => Self::Float(f1 + f2),
            (Self::Integer(i), Self::Float(f)) => Self::Float(i as f64 + f),
            (Self::Float(f), Self::Integer(i)) => Self::Float(f + i as f64),
            (Self::Integer(i1), Self::Integer(i2)) => Self::Integer(i1 + i2),
            _ => Self::Impossible,
        }
    }
}

impl Sub for Expr {
    type Output = Self;
    fn sub(self, by: Self) -> Self::Output {
        match (self, by) {
            (Self::Float(f1), Self::Float(f2)) => Self::Float(f1 - f2),
            (Self::Integer(i), Self::Float(f)) => Self::Float(i as f64 - f),
            (Self::Float(f), Self::Integer(i)) => Self::Float(f - i as f64),
            (Self::Integer(i1), Self::Integer(i2)) => Self::Integer(i1 - i2),
            _ => Self::Impossible,
        }
    }
}

impl Mul for Expr {
    type Output = Self;
    fn mul(self, by: Self) -> Self::Output {
        match (self, by) {
            (Self::Float(f1), Self::Float(f2)) => Self::Float(f1 * f2),
            (Self::Integer(i), Self::Float(f)) => Self::Float(i as f64 * f),
            (Self::Float(f), Self::Integer(i)) => Self::Float(f * i as f64),
            (Self::Integer(i1), Self::Integer(i2)) => Self::Integer(i1 * i2),
            _ => Self::Impossible,
        }
    }
}

impl Div for Expr {
    type Output = Self;
    fn div(self, by: Self) -> Self::Output {
        match (self, by) {
            (Self::Float(f1), Self::Float(f2)) => Self::Float(f1 / f2),
            (Self::Integer(i), Self::Float(f)) => Self::Float(i as f64 / f),
            (Self::Float(f), Self::Integer(i)) => Self::Float(f / i as f64),
            (Self::Integer(i1), Self::Integer(i2)) => Self::Float(i1 as f64 / i2 as f64),
            _ => Self::Impossible,
        }
    }
}

impl Rem for Expr {
    type Output = Self;
    fn rem(self, by: Self) -> Self::Output {
        match (self, by) {
            (Self::Float(f1), Self::Float(f2)) => Self::Float(f1 % f2),
            (Self::Integer(i), Self::Float(f)) => Self::Float(i as f64 % f),
            (Self::Float(f), Self::Integer(i)) => Self::Float(f % i as f64),
            (Self::Integer(i1), Self::Integer(i2)) => Self::Integer(i1 % i2),
            _ => Self::Impossible,
        }
    }
}

impl Expr {
    pub fn pow(self, by: Expr) -> Expr {
        match (self, by) {
            (Self::Float(f1), Self::Float(f2)) => Self::Float(f1.powf(f2)),
            (Self::Integer(i), Self::Float(f)) => Self::Float((i as f64).powf(f)),
            (Self::Float(f), Self::Integer(i)) => Self::Float(f.powf(i as f64)),
            (Self::Integer(i1), Self::Integer(i2)) => Self::Float((i1 as f64).powf(i2 as f64)),
            _ => Self::Impossible,
        }
    }

    pub fn as_bool(&self) -> Result<bool, Error> {
        if let Expr::Boolean(b) = self {
            Ok(*b)
        } else {
            Err(Error::EvalNotBool)
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mul => write!(f, "*"),
            Op::Div => write!(f, "/"),
            Op::Pow => write!(f, "^"),
            Op::Rem => write!(f, "%"),
            Op::Equals => write!(f, "=="),
            Op::NotEquals => write!(f, "!="),
            Op::Greater => write!(f, ">"),
            Op::Less => write!(f, "<"),
            Op::GreaterEq => write!(f, ">="),
            Op::LessEq => write!(f, "<="),
            Op::Or => write!(f, "or"),
            Op::And => write!(f, "and"),
            Op::In => write!(f, "in"),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Integer(i) => write!(f, "{}", i),
            Expr::Float(fl) => write!(f, "{}", fl),
            Expr::Boolean(b) => write!(f, "{}", b),
            Expr::Identifier(i) => write!(f, "{}", i.join(".")),
            Expr::VarIndex(i, l) => write!(
                f,
                "{}{}",
                i,
                l.iter()
                    .map(|x| format!("[{}]", x))
                    .collect::<Vec<_>>()
                    .join("")
            ),
            Expr::Exclusive(s, e) => write!(f, "{}..{}", s, e),
            Expr::Inclusive(s, e) => write!(f, "{}...{}", s, e),
            Expr::StringRaw(s) => write!(f, "{}", s),
            Expr::StringFmt(p) => write!(f, "{:?}", p),
            Expr::Array(s) => write!(
                f,
                "[{}]",
                s.iter()
                    .map(|i| format!("{}", i))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Expr::Table(s) => write!(f, "{}", s),
            Expr::FnCall(i, a) => write!(
                f,
                "{}({})",
                i,
                a.iter()
                    .map(|i| format!("{}", i))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Expr::BinOp(l, o, r) => write!(f, "{} {} {}", l, o, r),
            Expr::Not(e) => write!(f, "not {}", e),
            _ => unreachable!(),
        }
    }
}
