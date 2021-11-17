use std::io::ErrorKind;

use crate::*;
use crate::ast::{Function};


pub trait Compile {
    type Output;

    fn from_ast(ast: Result<Vec<Function>, std::io::Error>) -> Self::Output;

    fn from_source(source: &str) -> Self::Output {
        let parser = parser::CodeParser::default();
        let source = source
            .replace("\r", "")
            .replace("\n", "");
        println!("Compiling source:\n  {}", source);
        let ast = parser.parse(&source);
        println!("  {:?}", ast);
        Self::from_ast(ast)
    }
}

pub struct Interpreter;

impl Compile for Interpreter {
    type Output = Result<i32, std::io::Error>;

    fn from_ast(ast: Result<Vec<Function>, std::io::Error>) -> Self::Output {
        let evaluator = parser::Eval::default();
        match ast {
            Ok(ast) => {
                for function in ast {
                    for vars in function.vars {
                        evaluator.eval(&vars.expr);
                    }
                    for expr in function.exprs{
                        evaluator.eval(&expr);
                    }
                }
                Ok(0)
            }
            Err(e) => {
                Err(std::io::Error::new(ErrorKind::InvalidInput, e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::interpreter::{Interpreter, Compile};

    #[test]
    fn adding_two_numbers() {
        assert!(Interpreter::from_source("fn main(name: int){let s = 1 + 2;}").is_ok());
    }

    #[test]
    fn subtracting_two_numbers() {
        assert!(Interpreter::from_source("fn main(){let s = 3 - 1;}").is_ok());
    }

    #[test]
    fn increasing_a_number() {
        assert!(Interpreter::from_source("fn main(){let s = 2; s++;}").is_ok());
    }

    #[test]
    fn decreasing_a_number() {
        assert!(Interpreter::from_source("fn main(a:int){let s=3;s--;}").is_ok());
    }

    #[test]
    fn multiplying_two_numbers() {
        assert!(Interpreter::from_source("fn main(){let s = 3*2;}").is_ok());
    }

    #[test]
    fn dividing_two_numbers() {
        assert!(Interpreter::from_source("fn main(){let s = 6/2;}").is_ok());
    }

    #[test]
    fn creating_a_variable() {
        assert!(Interpreter::from_source("fn main(){
        let s = 2;
        }").is_ok());
    }

    #[test]
    fn requiring_a_main() {
        assert!(Interpreter::from_source("").is_err());
        assert!(Interpreter::from_source("let s = 2;").is_err());
        assert!(Interpreter::from_source("fn foo(){let s = 2;}").is_err());
        assert!(Interpreter::from_source("fn main(){let s = 2;}").is_ok());
    }

    #[test]
    fn reading_from_rs_file() {
        let file = std::fs::read_to_string("./static/example.rs").unwrap();
        assert!(Interpreter::from_source(&file).is_ok());
    }

    #[test]
    fn printing_from_rs_file() {
        let file = std::fs::read_to_string("./static/example_print.rs").unwrap();
        assert!(Interpreter::from_source(&file).is_ok());
    }
}
