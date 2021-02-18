use crate::{do_parse, do_walk, Action, Block, Error, Expr, IfPart, Namespace, Op, Table};
use smartstring::alias::String;
use std::collections::VecDeque;
use std::fs;
use std::io::{self, Write};

#[derive(Debug)]
pub struct Interpreter<'a> {
    pub env: Namespace<'a>,
    pub return_value: Option<Expr>,
    pub do_break: bool,
    pub do_continue: bool,
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        Self {
            env: Namespace::new(),
            return_value: None,
            do_break: false,
            do_continue: false,
        }
    }

    pub fn run_block(&mut self, block: &[Action]) -> Result<(), Error> {
        for act in block {
            self.run(act)?;
            if self.return_value.is_some() || self.do_continue || self.do_break {
                break;
            }
        }
        Ok(())
    }

    pub fn run(&mut self, act: &Action) -> Result<(), Error> {
        match act {
            Action::Comment(_) => Ok(()),
            Action::VarAssign(id, val) => self.var_assign(id, val),
            Action::SingleShot(id, op, val) => self.single_shot(id, op, val),
            Action::If(parts) => self.if_stmt(parts),
            Action::FnDef(id, args, block) => {
                self.fn_def(id, args, block.to_owned());
                Ok(())
            }
            Action::FnCall(id, args) => {
                self.fn_call(id, args)?;
                self.return_value = None;
                Ok(())
            }
            Action::Return(value) => {
                if self.env.in_function() {
                    self.return_value =
                        Some(self.expr(value.as_ref().unwrap_or(&Expr::Boolean(false)))?);
                    Ok(())
                } else {
                    Err(Error::ReturnOutOfFunction)
                }
            }
            Action::Loop(block) => self.loop_stmt(block),
            Action::For(id, expr, block) => self.for_stmt(id, expr, block),
            Action::While(expr, block) => self.while_stmt(expr, block),
            Action::Break => self.break_stmt(),
            Action::Continue => self.continue_stmt(),
            Action::Import(id, into) => self.import_stmt(id, into),
        }
    }

    fn import_stmt(&mut self, id: &Expr, _into: &Option<Expr>) -> Result<(), Error> {
        if let Expr::Identifier(parts) = id {
            let raw = fs::read_to_string(format!("{}.psi", parts[0].to_string()))?;
            let parse = do_parse(&raw)?;
            let walk = do_walk(parse)?;
            self.run_block(&walk)?;
        }
        Ok(())
    }

    fn while_stmt(&mut self, expr: &Expr, block: &[Action]) -> Result<(), Error> {
        self.env.enter("while");
        while !self.do_break
            && self.expr(expr)? == Expr::Boolean(true)
            && self.return_value.is_none()
        {
            self.run_block(block)?;
        }
        self.do_break = false;
        self.do_continue = false;
        self.env.leave();
        Ok(())
    }

    fn for_stmt(&mut self, id: &[String], expr: &Expr, block: &[Action]) -> Result<(), Error> {
        let expr = self.expr(expr)?;
        self.env.enter("for");
        let mut iteration = 0;
        while !self.do_break && self.return_value.is_none() {
            if let Ok(this) = self.var_index(&expr, vec![Expr::Integer(iteration)].into()) {
                self.env.raw_expr(&id[0], &this);
                self.run_block(block)?;
                iteration += 1;
            } else if let Expr::Table(ref items) = expr {
                if id.len() != 2 {
                    return Err(Error::IndexError);
                }
                self.env.raw_expr(
                    &id[0],
                    if let Some(i) = items.keys().get(iteration as usize) {
                        i
                    } else {
                        break;
                    },
                );
                self.env.raw_expr(
                    &id[1],
                    if let Some(i) = items.values().get(iteration as usize) {
                        i
                    } else {
                        break;
                    },
                );
                self.run_block(block)?;
                iteration += 1;
            } else {
                break;
            }
        }
        self.do_break = false;
        self.do_continue = false;
        self.env.leave();
        Ok(())
    }

    fn break_stmt(&mut self) -> Result<(), Error> {
        if self.env.in_loop() {
            self.do_break = true;
            Ok(())
        } else {
            Err(Error::BreakOutOfLoop)
        }
    }

    fn continue_stmt(&mut self) -> Result<(), Error> {
        if self.env.in_loop() {
            self.do_continue = true;
            Ok(())
        } else {
            Err(Error::ContinueOutOfLoop)
        }
    }

    fn loop_stmt(&mut self, block: &[Action]) -> Result<(), Error> {
        self.env.enter("loop");
        while !self.do_break {
            self.run_block(block)?;
        }
        self.do_break = false;
        self.env.leave();
        Ok(())
    }

    fn fn_def(&mut self, id: &Expr, args: &[String], block: Block) {
        if let Expr::Identifier(id) = id {
            self.env.insert_fn(&id.join("."), args, &block);
        }
    }

    fn if_stmt(&mut self, parts: &[IfPart]) -> Result<(), Error> {
        for part in parts {
            match part {
                IfPart::If(e, b) | IfPart::IfElse(e, b) => {
                    if let Expr::Boolean(true) = self.expr(e)? {
                        self.run_block(b)?;
                        break;
                    }
                }
                IfPart::Else(b) => self.run_block(b)?,
            }
        }
        Ok(())
    }

    fn single_shot(&mut self, id: &Expr, op: &Op, val: &Expr) -> Result<(), Error> {
        let exist = self.expr(id)?;
        if let Expr::Identifier(i) = id {
            let val = self.expr(val)?;
            let new = match op {
                Op::Add => exist + val,
                Op::Sub => exist - val,
                Op::Mul => exist * val,
                Op::Div => exist / val,
                Op::Rem => exist % val,
                Op::Pow => exist.pow(val),
                _ => unreachable!(),
            };
            self.env.insert_expr(&i.join("."), &new);
        }
        Ok(())
    }

    fn var_assign(&mut self, id: &Expr, val: &Expr) -> Result<(), Error> {
        let val = self.expr(val)?;
        if let Expr::Identifier(id) = id {
            self.env.insert_expr(&id.join("."), &val);
        }
        Ok(())
    }

    fn expr(&mut self, e: &Expr) -> Result<Expr, Error> {
        Ok(match e {
            Expr::Float(_) | Expr::Integer(_) | Expr::Boolean(_) | Expr::StringRaw(_) => {
                e.to_owned()
            }
            Expr::StringFmt(parts) => self.string_fmt(parts)?,
            Expr::Identifier(id) => self.identifier(id)?,
            Expr::Inclusive(s, e) => self.inclusive(s, e)?,
            Expr::Exclusive(s, e) => self.exclusive(s, e)?,
            Expr::Array(items) => self.array(items)?,
            Expr::Table(items) => self.table(items.to_owned())?,
            Expr::Not(e) => self.not(e)?,
            Expr::BinOp(left, op, right) => self.bin_op(*left.clone(), *op, *right.clone())?,
            Expr::FnCall(id, args) => {
                self.fn_call(id, args)?;
                let val = if let Some(val) = self.return_value.clone() {
                    val
                } else {
                    return Err(Error::NoReturnValue);
                };
                self.return_value = None;
                val
            }
            Expr::VarIndex(id, inds) => self.var_index(&*id, inds.to_owned())?,
            Expr::Impossible => return Err(Error::ImpossibleOperation),
        })
    }

    fn var_index(&mut self, id: &Expr, mut inds: VecDeque<Expr>) -> Result<Expr, Error> {
        let mut result = self.expr(id)?;
        while let Some(ind) = inds.pop_front() {
            let ind = self.expr(&ind)?;
            match result {
                Expr::StringRaw(s) => result = self.index_string(&s, &ind)?,
                Expr::Inclusive(s, e) => {
                    let (s, e, i) = if let Some(rp) = self.range_parts(&s, &e, &ind) {
                        rp
                    } else {
                        return Err(Error::InvalidIndex);
                    };
                    if s + i <= e {
                        return Ok(Expr::Integer(s + i));
                    } else {
                        return Err(Error::IndexError);
                    }
                }
                Expr::Exclusive(s, e) => {
                    let (s, e, i) = if let Some(rp) = self.range_parts(&s, &e, &ind) {
                        rp
                    } else {
                        return Err(Error::InvalidIndex);
                    };
                    if s + i < e {
                        return Ok(Expr::Integer(s + i));
                    } else {
                        return Err(Error::IndexError);
                    }
                }
                Expr::Array(items) => {
                    if let Expr::Integer(i) = ind {
                        result = if let Some(item) = items.get(i as usize) {
                            item.to_owned()
                        } else {
                            return Err(Error::IndexError);
                        }
                    } else {
                        return Err(Error::IndexError);
                    }
                }
                Expr::Table(items) => {
                    result = if let Some(ind) = items.get(ind) {
                        ind
                    } else {
                        return Err(Error::IndexError);
                    }
                }
                Expr::VarIndex(id, inds) => result = self.var_index(&id, inds)?,
                _ => return Err(Error::IndexError),
            }
        }
        Ok(result)
    }

    fn index_string(&mut self, s: &str, ind: &Expr) -> Result<Expr, Error> {
        if let Expr::Integer(i) = ind {
            Ok(Expr::StringRaw(
                if let Some(c) = s.chars().nth(*i as usize) {
                    c.to_string().into()
                } else {
                    return Err(Error::IndexError);
                },
            ))
        } else {
            Err(Error::InvalidIndex)
        }
    }

    fn range_parts(&self, s: &Expr, e: &Expr, ind: &Expr) -> Option<(i64, i64, i64)> {
        if let (Expr::Integer(s), Expr::Integer(e), Expr::Integer(i)) = (s, e, ind) {
            Some((*s, *e, *i))
        } else {
            None
        }
    }

    fn fn_call(&mut self, id: &Expr, args: &[Expr]) -> Result<(), Error> {
        self.return_value = None;
        let mut args_result = vec![];
        for i in args {
            args_result.push(self.expr(i)?);
        }
        let args = args_result;
        if let Expr::Identifier(ref parts) = id {
            self.env.enter("function");
            if let Some((needed_arguments, block)) = self.env.get_fn(&parts.join(".")) {
                for (k, v) in needed_arguments.iter().zip(&args) {
                    self.env.insert_expr(k, v);
                }
                self.run_block(&block)?;
            } else {
                match parts
                    .iter()
                    .map(|x| x.as_str())
                    .collect::<Vec<_>>()
                    .as_slice()
                {
                    ["print"] => println!("{}", args[0]),
                    ["input"] => {
                        let mut input = std::string::String::new();
                        print!("{}", args[0]);
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut input).unwrap();
                        self.return_value =
                            Some(Expr::StringRaw(input.trim_end_matches('\n').into()))
                    }
                    _ => return Err(Error::FunctionNotFound(parts.join(".").into())),
                }
            }
            self.env.leave();
        } else {
            unreachable!()
        }
        Ok(())
    }

    fn bin_op(&mut self, mut left: Expr, op: Op, mut right: Expr) -> Result<Expr, Error> {
        left = self.expr(&left)?;
        right = self.expr(&right)?;
        Ok(match op {
            Op::Add => left + right,
            Op::Sub => left - right,
            Op::Mul => left * right,
            Op::Div => left / right,
            Op::Pow => left.pow(right),
            Op::Rem => left % right,
            Op::Equals => Expr::Boolean(left == right),
            Op::NotEquals => Expr::Boolean(left != right),
            Op::Greater => Expr::Boolean(left > right),
            Op::Less => Expr::Boolean(left < right),
            Op::GreaterEq => Expr::Boolean(left >= right),
            Op::LessEq => Expr::Boolean(left <= right),
            Op::Or => Expr::Boolean(left.as_bool()? || right.as_bool()?),
            Op::And => Expr::Boolean(left.as_bool()? && right.as_bool()?),
            Op::In => self.in_expr(&left, &right)?,
        })
    }

    fn in_expr(&mut self, left: &Expr, right: &Expr) -> Result<Expr, Error> {
        let (left, right) = (self.expr(left)?, self.expr(right)?);
        Ok(match right {
            Expr::StringRaw(fish) => {
                if let Expr::StringRaw(sea) = left {
                    Expr::Boolean(fish.contains(sea.as_str()))
                } else {
                    return Err(Error::InvalidIndex);
                }
            }
            Expr::Inclusive(s, e) => self.in_inc(&s, &e, &left)?,
            Expr::Exclusive(s, e) => self.in_exc(&s, &e, &left)?,
            Expr::Array(ref items) => Expr::Boolean(items.contains(&left)),
            Expr::Table(ref items) => Expr::Boolean(items.contains(&left)),
            _ => return Err(Error::InvalidIndex),
        })
    }

    fn in_inc(&mut self, s: &Expr, e: &Expr, left: &Expr) -> Result<Expr, Error> {
        let (s, e) = (self.expr(s)?, self.expr(e)?);
        if let (Expr::Integer(s), Expr::Integer(e), Expr::Integer(l)) = (s, e, left) {
            Ok(Expr::Boolean(l <= &e && l >= &s))
        } else {
            Err(Error::InvalidIndex)
        }
    }

    fn in_exc(&mut self, s: &Expr, e: &Expr, left: &Expr) -> Result<Expr, Error> {
        let (s, e) = (self.expr(s)?, self.expr(e)?);
        if let (Expr::Integer(s), Expr::Integer(e), Expr::Integer(l)) = (s, e, left) {
            Ok(Expr::Boolean(l < &e && l >= &s))
        } else {
            Err(Error::InvalidIndex)
        }
    }

    fn not(&mut self, e: &Expr) -> Result<Expr, Error> {
        if let Expr::Boolean(b) = self.expr(e)? {
            Ok(Expr::Boolean(!b))
        } else {
            Err(Error::EvalNotBool)
        }
    }

    fn array(&mut self, items: &[Expr]) -> Result<Expr, Error> {
        let mut items_result = vec![];
        for i in items {
            items_result.push(self.expr(i)?);
        }
        Ok(Expr::Array(items_result))
    }

    fn table(&mut self, mut items: Table) -> Result<Expr, Error> {
        items.simplify(&mut |x| self.expr(&x))?;
        Ok(Expr::Table(items))
    }

    fn inclusive(&mut self, start: &Expr, end: &Expr) -> Result<Expr, Error> {
        Ok(Expr::Inclusive(
            Box::new(self.expr(start)?),
            Box::new(self.expr(end)?),
        ))
    }

    fn exclusive(&mut self, start: &Expr, end: &Expr) -> Result<Expr, Error> {
        Ok(Expr::Exclusive(
            Box::new(self.expr(start)?),
            Box::new(self.expr(end)?),
        ))
    }

    fn string_fmt(&mut self, parts: &[Expr]) -> Result<Expr, Error> {
        let mut result = String::new();
        for part in parts {
            result.push_str(&format!("{}", self.expr(part)?));
        }
        Ok(Expr::StringRaw(result))
    }

    fn identifier(&mut self, id: &[String]) -> Result<Expr, Error> {
        if let Some(e) = self.env.get_expr(&id.join(".")) {
            Ok(e)
        } else {
            Err(Error::VariableNotFound(id.join(".").into()))
        }
    }
}

impl<'a> Default for Interpreter<'a> {
    fn default() -> Self {
        Self::new()
    }
}
