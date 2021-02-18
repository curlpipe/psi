#![allow(clippy::upper_case_acronyms)]

#[macro_use]
extern crate quick_error;
extern crate pest;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod enums;
mod error;
mod expr;
mod interpreter;
mod namespace;
mod table;

pub use crate::enums::{Action, Block, Identifier, Identifiers, IfPart};
pub use crate::error::Error;
pub use crate::expr::{Expr, Op};
pub use crate::interpreter::Interpreter;
use crate::namespace::Namespace;
use crate::table::Table;
use pest_consume::{match_nodes, Error as PCError, Parser};
use smartstring::alias::String;

#[derive(Parser)]
#[grammar = "psi.pest"]
pub struct PsiParser;

type Pesult<T> = std::result::Result<T, PCError<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

pub fn do_walk(node: Node) -> Result<Vec<Action>, Error> {
    let instructions = PsiParser::stmts(node)?;
    Ok(instructions)
}

pub fn do_parse(src: &str) -> Result<Node, Error> {
    let parse = PsiParser::parse(Rule::stmts, src)?;
    let input = parse.single()?;
    Ok(input)
}

#[pest_consume::parser]
impl PsiParser {
    fn EOI(_input: Node) -> Pesult<()> {
        Ok(())
    }
    fn positive(_input: Node) -> Pesult<()> {
        Ok(())
    }
    fn negative(_input: Node) -> Pesult<()> {
        Ok(())
    }
    fn equals(_input: Node) -> Pesult<()> {
        Ok(())
    }
    fn not_equals(_input: Node) -> Pesult<()> {
        Ok(())
    }
    fn greater(_input: Node) -> Pesult<()> {
        Ok(())
    }
    fn less(_input: Node) -> Pesult<()> {
        Ok(())
    }
    fn greater_eq(_input: Node) -> Pesult<()> {
        Ok(())
    }
    fn less_eq(_input: Node) -> Pesult<()> {
        Ok(())
    }
    fn and(_input: Node) -> Pesult<()> {
        Ok(())
    }
    fn or(_input: Node) -> Pesult<()> {
        Ok(())
    }
    fn continue_stmt(_input: Node) -> Pesult<()> {
        Ok(())
    }
    fn break_stmt(_input: Node) -> Pesult<()> {
        Ok(())
    }

    fn identifier_part(input: Node) -> Pesult<&str> {
        Ok(input.as_str())
    }

    fn identifier(input: Node) -> Pesult<Expr> {
        Ok(match_nodes!(input.into_children();
            [identifier_part(part)..] => Expr::Identifier(part.map(|x| x.into()).collect()),
        ))
    }

    fn boolean(input: Node) -> Pesult<Expr> {
        let data = input.as_str().parse().map_err(|e| input.error(e));
        Ok(Expr::Boolean(data?))
    }

    fn float(input: Node) -> Pesult<Expr> {
        let data = input.as_str().parse().map_err(|e| input.error(e));
        Ok(Expr::Float(data?))
    }

    fn integer(input: Node) -> Pesult<Expr> {
        let data = input.as_str().parse().map_err(|e| input.error(e));
        Ok(Expr::Integer(data?))
    }

    fn string_esc(input: Node) -> Pesult<&str> {
        Ok(match_nodes!(input.into_children();
            [string_brack] => "{",
            [string_quote] => "\"",
        ))
    }

    fn string_expr(input: Node) -> Pesult<Expr> {
        Ok(match_nodes!(input.into_children();
            [expr(e)] => e,
        ))
    }

    fn string_char(input: Node) -> Pesult<&str> {
        Ok(input.as_str())
    }

    fn string_part(input: Node) -> Pesult<Expr> {
        Ok(match_nodes!(input.into_children();
            [string_esc(e)] => Expr::StringRaw(e.into()),
            [string_expr(e)] => e,
            [string_char(c)] => Expr::StringRaw(c.into()),
        ))
    }

    fn string(input: Node) -> Pesult<Expr> {
        Ok(match_nodes!(input.into_children();
            [string_part(parts)..] => Expr::StringFmt(parts.collect()),
        ))
    }

    fn exclusive(input: Node) -> Pesult<Expr> {
        Ok(match_nodes!(input.into_children();
            [bound(start), bound(end)] => Expr::Exclusive(Box::new(start), Box::new(end)),
        ))
    }

    fn inclusive(input: Node) -> Pesult<Expr> {
        Ok(match_nodes!(input.into_children();
            [bound(start), bound(end)] => Expr::Inclusive(Box::new(start), Box::new(end)),
        ))
    }

    fn table_pair(input: Node) -> Pesult<(Expr, Expr)> {
        Ok(match_nodes!(input.into_children();
            [datatype(d), expr(e)] => (d, e),
        ))
    }

    fn table(input: Node) -> Pesult<Expr> {
        Ok(match_nodes!(input.into_children();
            [table_pair(a)..] => Expr::Table(Table::new(a.collect())),
        ))
    }

    fn array(input: Node) -> Pesult<Expr> {
        Ok(match_nodes!(input.into_children();
            [expr(a)..] => Expr::Array(a.collect()),
        ))
    }

    fn bound(input: Node) -> Pesult<Expr> {
        // bound = { "(" ~ expr ~ ")" | sign | integer | identifier }
        Ok(match_nodes!(input.into_children();
            [expr(e)] => e,
            [sign(s)] => s,
            [integer(i)] => i,
            [identifier(i)] => i,
        ))
    }

    fn datatype(input: Node) -> Pesult<Expr> {
        // datatype = { boolean | array | table | inclusive |
        //              exclusive | float | integer | string }
        Ok(match_nodes!(input.into_children();
            [boolean(e)] => e,
            [array(d)] => d,
            [table(s)] => s,
            [inclusive(i)] => i,
            [exclusive(e)] => e,
            [float(f)] => f,
            [integer(i)] => i,
            [string(s)] => s,
        ))
    }

    fn var_index(input: Node) -> Pesult<Expr> {
        // var_index = { atom ~ ("[" ~ expr ~ "]")* }
        Ok(match_nodes!(input.into_children();
            [atom(id)] => id,
            [atom(id), expr(e)..] => Expr::VarIndex(Box::new(id), e.collect()),
        ))
    }

    fn atom(input: Node) -> Pesult<Expr> {
        // atom = { datatype | "(" ~ expr ~ ")" | sign | identifier }
        Ok(match_nodes!(input.into_children();
            [expr(e)] => e,
            [datatype(d)] => d,
            [sign(s)] => s,
            [identifier(i)] => i,
        ))
    }

    fn sign(input: Node) -> Pesult<Expr> {
        // sign = { (positive | negative) ~ (float | integer | sign) }
        Ok(match_nodes!(input.into_children();
            [positive(_), float(f)] => f,
            [positive(_), integer(i)] => i,
            [positive(_), sign(s)] => s,
            [negative(_), float(f)] =>
                Expr::BinOp(Box::new(f), Op::Mul, Box::new(Expr::Integer(-1))),
            [negative(_), integer(i)] =>
                Expr::BinOp(Box::new(i), Op::Mul, Box::new(Expr::Integer(-1))),
            [negative(_), sign(s)] =>
                Expr::BinOp(Box::new(s), Op::Mul, Box::new(Expr::Integer(-1))),
        ))
    }

    fn pow(input: Node) -> Pesult<Expr> {
        // pow = { var_index ~ ("^" ~ var_index)* }
        Ok(match_nodes!(input.into_children();
            [var_index(left)] => left,
            [var_index(mut nums)..] => {
                let mut result: Option<Expr> = None;
                while let Some(p) = nums.next_back() {
                    if let Some(i) = result {
                        result = Some(Expr::BinOp(Box::new(p), Op::Pow, Box::new(i)));
                    } else {
                        result = Some(p);
                    }
                }
                result.unwrap()
            }
        ))
    }

    fn rem(input: Node) -> Pesult<Expr> {
        // rem = { pow ~ ("%" ~ pow)* }
        Ok(match_nodes!(input.into_children();
            [pow(left)] => left,
            [pow(mut nums)..] => {
                let mut result = None;
                while let Some(p) = nums.next() {
                    if let Some(i) = result {
                        result = Some(Expr::BinOp(Box::new(i), Op::Rem, Box::new(p)));
                    } else {
                        result = Some(p);
                    }
                }
                result.unwrap()
            }
        ))
    }

    fn div(input: Node) -> Pesult<Expr> {
        // div = { rem ~ ("/" ~ rem)* }
        Ok(match_nodes!(input.into_children();
            [rem(left)] => left,
            [rem(mut nums)..] => {
                let mut result = None;
                while let Some(p) = nums.next() {
                    if let Some(i) = result {
                        result = Some(Expr::BinOp(Box::new(i), Op::Div, Box::new(p)));
                    } else {
                        result = Some(p);
                    }
                }
                result.unwrap()
            }
        ))
    }

    fn mul(input: Node) -> Pesult<Expr> {
        // mul = { div ~ ("*" ~ div)* }
        Ok(match_nodes!(input.into_children();
            [div(left)] => left,
            [div(mut nums)..] => {
                let mut result = None;
                while let Some(p) = nums.next() {
                    if let Some(i) = result {
                        result = Some(Expr::BinOp(Box::new(i), Op::Mul, Box::new(p)));
                    } else {
                        result = Some(p);
                    }
                }
                result.unwrap()
            }
        ))
    }

    fn add(input: Node) -> Pesult<Expr> {
        // add = { mul ~ ("+" ~ mul)* }
        Ok(match_nodes!(input.into_children();
            [mul(left)] => left,
            [mul(mut nums)..] => {
                let mut result = None;
                while let Some(p) = nums.next() {
                    if let Some(i) = result {
                        result = Some(Expr::BinOp(Box::new(i), Op::Add, Box::new(p)));
                    } else {
                        result = Some(p);
                    }
                }
                result.unwrap()
            }
        ))
    }

    fn sub(input: Node) -> Pesult<Expr> {
        // sub = { add ~ ("-" ~ add)* }
        Ok(match_nodes!(input.into_children();
            [add(left)] => left,
            [add(mut nums)..] => {
                let mut result = None;
                while let Some(p) = nums.next() {
                    if let Some(i) = result {
                        result = Some(Expr::BinOp(Box::new(i), Op::Sub, Box::new(p)));
                    } else {
                        result = Some(p);
                    }
                }
                result.unwrap()
            }
        ))
    }

    fn ss_add(input: Node) -> Pesult<Action> {
        Ok(match_nodes!(input.into_children();
            [identifier(i), expr(e)] => Action::SingleShot(i, Op::Add, e)
        ))
    }

    fn ss_sub(input: Node) -> Pesult<Action> {
        Ok(match_nodes!(input.into_children();
            [identifier(i), expr(e)] => Action::SingleShot(i, Op::Sub, e)
        ))
    }

    fn ss_mul(input: Node) -> Pesult<Action> {
        Ok(match_nodes!(input.into_children();
            [identifier(i), expr(e)] => Action::SingleShot(i, Op::Mul, e)
        ))
    }

    fn ss_div(input: Node) -> Pesult<Action> {
        Ok(match_nodes!(input.into_children();
            [identifier(i), expr(e)] => Action::SingleShot(i, Op::Div, e)
        ))
    }

    fn ss_rem(input: Node) -> Pesult<Action> {
        Ok(match_nodes!(input.into_children();
            [identifier(i), expr(e)] => Action::SingleShot(i, Op::Rem, e)
        ))
    }

    fn ss_pow(input: Node) -> Pesult<Action> {
        Ok(match_nodes!(input.into_children();
            [identifier(i), expr(e)] => Action::SingleShot(i, Op::Pow, e)
        ))
    }

    fn single_shot(input: Node) -> Pesult<Action> {
        Ok(match_nodes!(input.into_children();
            [ss_add(e)] => e,
            [ss_sub(e)] => e,
            [ss_mul(e)] => e,
            [ss_div(e)] => e,
            [ss_rem(e)] => e,
            [ss_pow(e)] => e,
        ))
    }

    fn comp(input: Node) -> Pesult<Expr> {
        // comp = { sub ~ ((greater_eq | less_eq | greater | less) ~ sub)? }
        Ok(match_nodes!(input.into_children();
            [sub(left)] => left,
            [sub(left), greater_eq(_), sub(right)] =>
                Expr::BinOp(Box::new(left), Op::GreaterEq, Box::new(right)),
            [sub(left), less_eq(_), sub(right)] =>
                Expr::BinOp(Box::new(left), Op::LessEq, Box::new(right)),
            [sub(left), greater(_), sub(right)] =>
                Expr::BinOp(Box::new(left), Op::Greater, Box::new(right)),
            [sub(left), less(_), sub(right)] =>
                Expr::BinOp(Box::new(left), Op::Less, Box::new(right)),
        ))
    }

    fn eq(input: Node) -> Pesult<Expr> {
        // eq = { comp ~ ((equals | not_equals) ~ comp)? }
        Ok(match_nodes!(input.into_children();
            [comp(left)] => left,
            [comp(left), equals(_), comp(right)] =>
                Expr::BinOp(Box::new(left), Op::Equals, Box::new(right)),
            [comp(left), not_equals(_), comp(right)] =>
                Expr::BinOp(Box::new(left), Op::NotEquals, Box::new(right)),
        ))
    }

    fn comb(input: Node) -> Pesult<Expr> {
        // comb = { eq ~ ((and | or) ~ eq)? }
        Ok(match_nodes!(input.into_children();
            [eq(left)] => left,
            [eq(left), and(_), eq(right)] =>
                Expr::BinOp(Box::new(left), Op::And, Box::new(right)),
            [eq(left), or(_), eq(right)] =>
                Expr::BinOp(Box::new(left), Op::Or, Box::new(right)),
        ))
    }

    fn not(input: Node) -> Pesult<Expr> {
        Ok(match_nodes!(input.into_children();
            [expr(e)] => Expr::Not(Box::new(e)),
        ))
    }

    fn iterable(input: Node) -> Pesult<Expr> {
        // iterable = { array | table | var_index | inclusive | exclusive | identifier }
        Ok(match_nodes!(input.into_children();
            [array(a)] => a,
            [table(t)] => t,
            [var_index(v)] => v,
            [inclusive(i)] => i,
            [exclusive(e)] => e,
            [identifier(i)] => i,
        ))
    }

    fn contains(input: Node) -> Pesult<Expr> {
        Ok(match_nodes!(input.into_children();
            [atom(a), iterable(i)] => Expr::BinOp(Box::new(a), Op::In, Box::new(i)),
        ))
    }

    fn expr(input: Node) -> Pesult<Expr> {
        Ok(match_nodes!(input.into_children();
            [integer(int)] => int,
            [comb(com)] => com,
            [not(no)] => no,
            [contains(con)] => con,
            [fn_call(call)] => call.to_expr(),
        ))
    }

    fn comment(input: Node) -> Pesult<Action> {
        Ok(Action::Comment(input.as_str().into()))
    }

    fn fn_call(input: Node) -> Pesult<Action> {
        Ok(match_nodes!(input.into_children();
            [identifier(i), expr(e)..] => Action::FnCall(i, e.collect()),
        ))
    }

    fn var_assign(input: Node) -> Pesult<Action> {
        Ok(match_nodes!(input.into_children();
            [identifier(id), expr(exp)] => Action::VarAssign(id, exp),
        ))
    }

    fn block(input: Node) -> Pesult<Vec<Action>> {
        Ok(match_nodes!(input.into_children();
            [stmt(s)..] => s.collect(),
        ))
    }

    fn if_stmt(input: Node) -> Pesult<IfPart> {
        Ok(match_nodes!(input.into_children();
            [expr(e), block(b)] => IfPart::If(e, b),
        ))
    }

    fn else_if_stmt(input: Node) -> Pesult<IfPart> {
        Ok(match_nodes!(input.into_children();
            [expr(e), block(b)] => IfPart::IfElse(e, b),
        ))
    }

    fn else_stmt(input: Node) -> Pesult<IfPart> {
        Ok(match_nodes!(input.into_children();
            [block(b)] => IfPart::Else(b),
        ))
    }

    fn ifs(input: Node) -> Pesult<Action> {
        Ok(match_nodes!(input.into_children();
            [if_stmt(first)] => Action::If(vec![first]),
            [if_stmt(first), else_if_stmt(second)..] => {
                let mut result = vec![first];
                result.extend(second);
                Action::If(result)
            },
            [if_stmt(first), else_if_stmt(second).., else_stmt(third)] => {
                let mut result = vec![first];
                result.extend(second);
                result.push(third);
                Action::If(result)
            },
            [if_stmt(first), else_stmt(third)] => Action::If(vec![first, third]),
        ))
    }

    fn args(input: Node) -> Pesult<Vec<String>> {
        Ok(match_nodes!(input.into_children();
            [identifier_part(i)..] => i.map(|x| x.into()).collect(),
        ))
    }

    fn fn_def(input: Node) -> Pesult<Action> {
        Ok(match_nodes!(input.into_children();
            [identifier(i), args(a), block(b)] => Action::FnDef(i, a, b),
        ))
    }

    fn return_stmt(input: Node) -> Pesult<Action> {
        Ok(match_nodes!(input.into_children();
            [return_expr(e)] => Action::Return(e),
        ))
    }

    fn return_expr(input: Node) -> Pesult<Option<Expr>> {
        Ok(match_nodes!(input.into_children();
            [expr(e)] => Some(e),
            [] => None,
        ))
    }

    fn for_stmt(input: Node) -> Pesult<Action> {
        Ok(match_nodes!(input.into_children();
            [args(a), iterable(i), block(b)] => Action::For(a, i, b),
        ))
    }

    fn loop_stmt(input: Node) -> Pesult<Action> {
        Ok(match_nodes!(input.into_children();
            [block(b)] => Action::Loop(b),
        ))
    }

    fn while_stmt(input: Node) -> Pesult<Action> {
        Ok(match_nodes!(input.into_children();
            [expr(c), block(b)] => Action::While(c, b),
        ))
    }

    fn loop_control(input: Node) -> Pesult<Action> {
        Ok(match_nodes!(input.into_children();
            [break_stmt(_)] => Action::Break,
            [continue_stmt(_)] => Action::Continue,
        ))
    }

    fn import_stmt(input: Node) -> Pesult<Action> {
        Ok(match_nodes!(input.into_children();
            [identifier(from), identifier(into)] =>
                Action::Import(from, Some(into)),
            [identifier(from)] => Action::Import(from, None),
        ))
    }

    fn stmt(input: Node) -> Pesult<Action> {
        Ok(match_nodes!(input.into_children();
            [var_assign(var)] => var,
            [comment(com)] => com,
            [fn_call(call)] => call,
            [fn_def(def)] => def,
            [return_stmt(ret)] => ret,
            [ifs(if_s)] => if_s,
            [for_stmt(for_s)] => for_s,
            [loop_stmt(loop_s)] => loop_s,
            [while_stmt(loop_s)] => loop_s,
            [single_shot(ss)] => ss,
            [loop_control(lc)] => lc,
            [import_stmt(imp)] => imp,
        ))
    }

    pub fn stmts(input: Node) -> Pesult<Vec<Action>> {
        Ok(match_nodes!(input.into_children();
            [stmt(act).., _] => act.collect(),
        ))
    }
}
