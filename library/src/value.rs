// value.rs - Representation and operations of data types
use std::ops::{Add, Sub, Mul, Div, Neg, Rem, BitXor};
use round::round;
use std::fmt;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        // Define how to print certain values
        match self {
            Self::Number(f) => write!(fmt, "{}", round(*f, 5)),
            Self::Boolean(b) => write!(fmt, "{}", b),
            Self::String(s) => write!(fmt, "{}", s),
            Self::Nil => write!(fmt, "nil"),
        }
    }
}

impl Add for Value {
    type Output = Value;
    fn add(self, other: Value) -> Self::Output {
        // Add two values
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a + b),
            (Self::String(a), Self::String(b)) => Self::String(a + &b),
            _ => unreachable!(),
        }
    }
}

impl Sub for Value {
    type Output = Value;
    fn sub(self, other: Value) -> Self::Output {
        // Subtract two values
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a - b),
            _ => unreachable!(),
        }
    }
}

impl Mul for Value {
    type Output = Value;
    fn mul(self, other: Value) -> Self::Output {
        // Multiply two values
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a * b),
            _ => unreachable!(),
        }
    }
}

impl Div for Value {
    type Output = Value;
    fn div(self, other: Value) -> Self::Output {
        // Divide two values
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a / b),
            _ => unreachable!(),
        }
    }
}

impl Rem for Value {
    type Output = Value;
    fn rem(self, other: Value) -> Self::Output {
        // Find the remainder of two values
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a % b),
            _ => unreachable!(),
        }
    }
}

// Actually acts as a power operator, not a bitxor
// We just want to trick rust into using it like this, for code clarity
impl BitXor for Value {
    type Output = Value;
    fn bitxor(self, other: Value) -> Self::Output {
        // Exponentiate two values
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a.powf(b)),
            _ => unreachable!(),
        }
    }
}

impl Neg for Value {
    type Output = Value;
    fn neg(self) -> Self::Output {
        // Negate a value
        match self {
            Self::Number(num) => Self::Number(-num),
            _ => unreachable!(),
        }
    }
}
