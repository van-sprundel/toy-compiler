use std::io::ErrorKind;
use pest::error::InputLocation;

use crate::ast::*;
use pest::Parser;
use crate::memory::Memory;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct PestParser;

#[derive(Default)]
pub(crate) struct CodeParser;

impl CodeParser {
    pub fn parse(&self, source: &str) -> std::result::Result<Vec<Function>, std::io::Error> {
        let mut ast = vec![];
        match PestParser::parse(Rule::Program, source) {
            Ok(pairs) => {
                let mut contains_main = false;
                for pair in pairs {
                    if let Rule::Function = pair.as_rule() {
                        let f = self.build_ast(pair);
                        if f.name == "main" {
                            contains_main = true;
                            let mut temp = vec![f];
                            temp.append(&mut ast);
                            ast = temp;
                        } else {
                            ast.push(f);
                        }
                    }
                }
                if contains_main {
                    Ok(ast)
                } else {
                    Err(std::io::Error::new(ErrorKind::InvalidInput, "Main function not found"))
                }
            }
            Err(e) => {
                if e.location == InputLocation::Pos(0) {
                    Err(std::io::Error::new(ErrorKind::InvalidInput, "Cannot parse file"))
                } else {
                    Err(std::io::Error::new(ErrorKind::InvalidInput, e.to_string()))
                }
            }
        }
    }
    pub fn build_ast(&self, pair: pest::iterators::Pair<Rule>) -> Function {
        match pair.as_rule() {
            Rule::Function => {
                let mut pair = pair.into_inner();
                let name = pair.next().unwrap();
                let a = pair.next().unwrap();
                let var;
                let arg = if let Rule::arg = a.as_rule() {
                    var = self.build_var_from_expr(pair.next().unwrap());
                    String::from(a.as_str())
                } else {
                    var = self.build_var_from_expr(a);
                    String::new()
                };
                let mut vars = vec![var];
                let mut exprs = vec![];
                let mut temp = pair.next();
                while temp.is_some() {
                    let p = temp.unwrap();
                    match p.as_rule() {
                        Rule::Variable => {
                            vars.push(self.build_var_from_expr(p));
                        }
                        _ => {
                            exprs.push(self.build_ast_from_expr(p));
                        }
                    }

                    temp = pair.next();
                }
                let ret = if let Some(pair) = temp {
                    panic!("{}", pair.as_str());
                } else {
                    0
                };
                Function {
                    name: String::from(name.as_str()),
                    args: String::from(arg.as_str()),
                    vars,
                    exprs,
                    ret,
                }
            }
            _ => unimplemented!()
        }
    }
    pub fn build_var_from_expr(&self, pair: pest::iterators::Pair<Rule>) -> Var {
        match pair.as_rule() {
            Rule::Variable => {
                let mut pair = pair.into_inner();
                let name = pair.next().unwrap();
                let res = pair.next().unwrap();
                let expr = self.build_ast_from_expr(res);
                Self::parse_variable(name, expr)
            }
            Rule::CrementBinary => {
                let mut pair = pair.into_inner();
                let lhs = pair.next().unwrap();
                let _op = pair.next().unwrap();
                Self::parse_variable(lhs, Expr::Literal(1))
            }
            _ => unimplemented!()
        }
    }
    pub fn build_ast_from_expr(&self, pair: pest::iterators::Pair<Rule>) -> Expr {
        match pair.as_rule() {
            Rule::Expr => self.build_ast_from_expr(pair.into_inner().next().unwrap()),
            Rule::Binary => {
                let mut pair = pair.into_inner();
                let lhspair = pair.next().unwrap();
                match pair.next() {
                    Some(p) => {
                        let lhs = self.build_ast_from_expr(lhspair);
                        let op = p;
                        let rhspair = pair.next().unwrap();
                        let rhs = self.build_ast_from_expr(rhspair);
                        CodeParser::parse_binary_expr(op, lhs, rhs)
                    }
                    None => {
                        let istr = lhspair.as_str();
                        let (sign, istr) = match &istr[..1] {
                            "-" => (-1, &istr[1..]),
                            _ => (1, istr),
                        };
                        let int: i32 = istr.parse().unwrap();
                        CodeParser::parse_term(sign, int)
                    }
                }
            }
            Rule::Term | Rule::Int => {
                let istr = pair.as_str();
                let (sign, istr) = match &istr[..1] {
                    "-" => (-1, &istr[1..]),
                    _ => (1, istr),
                };
                let int: i32 = istr.parse().unwrap();
                CodeParser::parse_term(sign, int)
            }
            Rule::Name => {
                Expr::Reference(pair.as_str().parse().unwrap())
            }
            Rule::Unary => {
                let mut pair = pair.into_inner();
                let op = pair.next().unwrap();
                let child = pair.next().unwrap();
                let child = self.build_ast_from_term(child);
                CodeParser::parse_unary_expr(op, child)
            }
            Rule::CrementBinary => {
                let mut pair = pair.into_inner();
                let lhspair = pair.next().unwrap();
                let lhs = self.build_ast_from_expr(lhspair);
                let op = pair.next().unwrap();
                CodeParser::parse_binary_expr(op, lhs, Expr::Literal(1))
            }
            Rule::FunctionCall => {
                Expr::Reference(pair.as_str().parse().unwrap())
            }
            unknown => panic!("Unknown expr: {:?}", unknown),
        }
    }
    fn build_ast_from_term(&self, pair: pest::iterators::Pair<Rule>) -> Expr {
        match pair.as_rule() {
            Rule::Int => {
                let istr = pair.as_str();
                let (sign, istr) = match &istr[..1] {
                    "-" => (-1, &istr[1..]),
                    _ => (1, istr),
                };
                let int: i32 = istr.parse().unwrap();
                Expr::Literal(sign * int)
            }
            Rule::Expr => self.build_ast_from_expr(pair),
            unknown => panic!("Unknown term: {:?}", unknown),
        }
    }

    fn parse_unary_expr(pair: pest::iterators::Pair<Rule>, child: Expr) -> Expr {
        Expr::Unary {
            op: match pair.as_str() {
                "+" => Operator::Add,
                "-" => Operator::Sub,
                _ => unimplemented!(),
            },
            child: Box::new(child),
        }
    }

    fn parse_binary_expr(pair: pest::iterators::Pair<Rule>, lhs: Expr, rhs: Expr) -> Expr {
        Expr::Binary {
            op: match pair.as_str() {
                "+" => Operator::Add,
                "++" => Operator::Incr,
                "--" => Operator::Decr,
                "-" => Operator::Sub,
                "*" => Operator::Mul,
                "/" => Operator::Div,
                _ => unimplemented!(),
            },
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
    fn parse_term(sign: i32, int: i32) -> Expr {
        Expr::Literal(sign * int)
    }
    fn parse_variable(name: pest::iterators::Pair<Rule>, expr: Expr) -> Var {
        Var {
            name: String::from(name.as_str()),
            expr,
        }
    }
}

pub struct Eval {
    pub memory: Memory,
}

impl Eval {
    pub fn new(memory: Memory) -> Self {
        Self { memory }
    }
    pub fn eval(&self, node: &Expr) -> i32 {
        match node {
            Expr::Literal(n) => *n,
            Expr::Unary { op, child } => {
                let child = self.eval(child);
                match op {
                    Operator::Add => child,
                    Operator::Sub => -child,
                    _ => unreachable!()
                }
            }
            Expr::Binary { op, lhs, rhs } => {
                let lhs_ret = self.eval(lhs);
                let rhs_ret = self.eval(rhs);

                let res = match op {
                    Operator::Add => lhs_ret.wrapping_add(rhs_ret),
                    Operator::Sub => lhs_ret.wrapping_sub(rhs_ret),
                    Operator::Mul => lhs_ret * rhs_ret,
                    Operator::Div => lhs_ret / rhs_ret,
                    Operator::Incr => lhs_ret.wrapping_add(1),
                    Operator::Decr =>lhs_ret.wrapping_sub(1),
                    Operator::Comp => !lhs_ret
                };
                println!("{:?} = {}", node, res);
                res
            }
            Expr::Reference(n) => {
                if let Ok(e) = self.memory.find(n) {
                    self.eval(&e)
                } else {
                    panic!("Couldn't find referenced variable");
                }
            }
        }
    }
}
