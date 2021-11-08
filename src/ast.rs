use std::fmt::{Debug, Formatter};

pub struct Var {
    pub(crate) name: String,
    pub(crate) expr: Expr
}

impl Debug for Var {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.expr {
            Expr::Unary { child,op } => {write!(f,"var {} = {:?}{:?}", &self.name,child,op)}
            Expr::Binary { lhs,op,rhs } => {write!(f, "var {} = {:?}, {:?}, {:?}",&self.name,lhs,op,rhs)}
            Expr::Literal(s) => {write!(f,"var {} = {}", &self.name, s)}
        }
    }
}

#[derive(Debug)]
pub enum Expr{
    Unary {
        child: Box<Expr>,
        op: Operator
    }, // { operator - term }
    Binary {
        lhs: Box<Expr>,
        op: Operator,
        rhs: Box<Expr>
    }, // { term - {operator - term)* }
    Literal(i32)
}
pub struct Literal{

}
#[derive(Debug)]
pub enum Operator {
    Add, // addition        | +     | binary | unary
    Sub, // subtraction     | -     | binary | unary
    Mul, // division        | *     | binary
    Div, // multiplication  | /     | binary
    Incr, // increase       | ++    | crement
    Decr, // decrease       | --    | crement
    Comp // complement      | !     | unary
}