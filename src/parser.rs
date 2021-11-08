use std::io::ErrorKind;
use pest::error::InputLocation;
use pest::iterators::Pair;
use crate::ast::*;
use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct PestParser;

#[derive(Default)]
pub(crate) struct CodeParser;

impl CodeParser {
    pub fn parse(&self, source: &str) -> std::result::Result<Vec<Var>, std::io::Error> {
        let mut ast = vec![];
        match PestParser::parse(Rule::Program, source) {
            Ok(pairs) => {
                for pair in pairs {
                    if let Rule::Variable = pair.as_rule() {
                        ast.push(self.build_ast(pair));
                    } else     if let Rule::CrementBinary = pair.as_rule() {
                        ast.push(self.build_ast(pair));
                    }

                }
                Ok(ast)
            }
            Err(e) => {
                if e.location == InputLocation::Pos(0) {
                    Err(std::io::Error::new(ErrorKind::InvalidInput, "Cannot find main function"))
                } else {
                    Err(std::io::Error::new(ErrorKind::InvalidInput,                 e.to_string()))
                }
            }
        }
    }
    pub fn build_ast(&self, pair: pest::iterators::Pair<Rule>) -> Var {
        match pair.as_rule() {
            Rule::Variable => {
                let mut pair = pair.into_inner();
                let name = pair.next().unwrap();
                let expr = self.build_ast_from_expr(pair.next().unwrap());
                Var {
                    name: String::from(name.as_str()),
                    expr,
                }
            },
            Rule::CrementBinary => {
                let mut pair = pair.into_inner();
                let lhs = pair.next().unwrap();
                let lhs = self.build_ast_from_term(lhs);
                let op = pair.next().unwrap();
                let expr = Self::parse_binary_expr(op, lhs, Expr::Literal(1));
                Var {
                    name: String::from("empty"),
                    expr,
                }
            }
            _ => unimplemented!()
        }
    }
    pub fn build_ast_from_expr(&self, pair: pest::iterators::Pair<Rule>) -> Expr {
        match pair.as_rule() {
            Rule::Expr => self.build_ast_from_expr(pair.into_inner().next().unwrap()),
            Rule::Term | Rule::Int => {
                let istr = pair.as_str();
                let (sign, istr) = match &istr[..1] {
                    "-" => (-1, &istr[1..]),
                    _ => (1, istr),
                };
                let int: i32 = istr.parse().unwrap();
                CodeParser::parse_term(sign, int)
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
                let lhs = pair.next().unwrap();
                let lhs = self.build_ast_from_term(lhs);
                let op = pair.next().unwrap();
                CodeParser::parse_binary_expr(op, lhs, Expr::Literal(1))
            }
            Rule::Binary => {
                let mut pair = pair.into_inner();
                let lhspair = pair.next().unwrap();
                let lhs = self.build_ast_from_term(lhspair);
                let op = pair.next().unwrap();
                let rhspair = pair.next().unwrap();
                let rhs = self.build_ast_from_term(rhspair);
                CodeParser::parse_binary_expr(op, lhs, rhs)
            }
            Rule::Variable => {
                let mut pair = pair.into_inner();
                // pair.next(); // var
                let name = pair.next().unwrap();
                // pair.next(); // =
                let expr = pair.next().unwrap();
                let expr = self.build_ast_from_expr(expr);
                Self::parse_variable(name, expr).expr
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

#[derive(Default)]
pub struct Eval;

impl Eval {
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

                match op {
                    Operator::Add => lhs_ret + rhs_ret,
                    Operator::Sub => lhs_ret - rhs_ret,
                    Operator::Mul => lhs_ret * rhs_ret,
                    Operator::Div => lhs_ret / rhs_ret,
                    Operator::Incr => lhs_ret + 1,
                    Operator::Decr => lhs_ret - 1,
                    Operator::Comp => !lhs_ret,
                }
            }
        }
    }
}
