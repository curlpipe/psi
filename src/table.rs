use crate::Expr;
use std::fmt;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Table {
    pub data: Vec<(Expr, Expr)>,
}

impl Table {
    pub fn new(data: Vec<(Expr, Expr)>) -> Self {
        Self { data }
    }

    pub fn simplify(&mut self, expr: &mut dyn FnMut(Expr) -> Expr) {
        let mut result = vec![];
        for (k, v) in self.data.clone() {
            result.push((expr(k), expr(v)));
        }
        self.data = result;
    }

    pub fn get(&self, index: Expr) -> Option<Expr> {
        for (k, v) in &self.data {
            if k == &index {
                return Some(v.to_owned());
            }
        }
        None
    }

    pub fn contains(&self, index: &Expr) -> bool {
        for (k, _) in &self.data {
            if index == k {
                return true;
            }
        }
        false
    }

    pub fn keys(&self) -> Vec<Expr> {
        self.data.iter().map(|(k, _)| k.clone()).collect()
    }

    pub fn values(&self) -> Vec<Expr> {
        self.data.iter().map(|(_, v)| v.clone()).collect()
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
