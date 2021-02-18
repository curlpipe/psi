use crate::{Action, Expr};
use rustc_hash::FxHashMap as HashMap;
use smartstring::alias::String;

const LOOP: [&str; 3] = ["loop", "for", "while"];

#[derive(Debug, Clone)]
pub enum Symbol {
    Function(Vec<String>, Vec<Action>),
    Expr(Expr),
}

#[derive(Debug)]
pub struct Namespace<'a> {
    env: Vec<(&'a str, HashMap<String, Symbol>)>,
}

impl<'a> Namespace<'a> {
    pub fn new() -> Self {
        Self {
            env: vec![("*", HashMap::default())],
        }
    }

    pub fn enter(&mut self, name: &'a str) {
        self.env.push((name, HashMap::default()));
    }

    pub fn leave(&mut self) {
        self.env.pop();
    }

    pub fn raw_expr(&mut self, id: &str, expr: &Expr) {
        let build = Symbol::Expr(expr.to_owned());
        let last = self.env.len() - 1;
        self.env.get_mut(last).unwrap().1.insert(id.into(), build);
    }

    pub fn insert_expr(&mut self, id: &str, expr: &Expr) {
        let build = Symbol::Expr(expr.to_owned());
        let mut banks = self.env.iter_mut();
        while let Some((name, contents)) = banks.next_back() {
            if ["*", "function"].contains(name) {
                contents.insert(id.into(), build);
                break;
            }
        }
    }

    pub fn insert_fn(&mut self, id: &str, args: &[String], block: &[Action]) {
        let build = Symbol::Function(args.to_owned(), block.to_owned());
        let mut banks = self.env.iter_mut();
        while let Some((name, contents)) = banks.next_back() {
            if ["*", "function"].contains(name) {
                contents.insert(id.into(), build);
                break;
            }
        }
    }

    pub fn get_expr(&mut self, id: &str) -> Option<Expr> {
        let mut banks = self.env.iter_mut();
        while let Some((name, contents)) = banks.next_back() {
            if let Some(Symbol::Expr(e)) = contents.get(id) {
                return Some(e.clone());
            }
            if name == &"function" {
                break;
            }
        }
        None
    }

    pub fn get_fn(&mut self, id: &str) -> Option<(Vec<String>, Vec<Action>)> {
        let mut banks = self.env.iter_mut();
        while let Some((_, contents)) = banks.next_back() {
            if let Some(Symbol::Function(a, b)) = contents.get(id) {
                return Some((a.clone(), b.clone()));
            }
        }
        None
    }

    pub fn in_function(&self) -> bool {
        self.env.iter().any(|(k, _)| k == &"function")
    }

    pub fn in_loop(&self) -> bool {
        self.env.iter().any(|(k, _)| LOOP.contains(k))
    }
}

impl<'a> Default for Namespace<'a> {
    fn default() -> Self {
        Self::new()
    }
}
