use std::io::ErrorKind;
use pest::error::Error;
use crate::*;
use crate::ast::Var;
use crate::parser::Rule;

pub trait Compile {
    type Output;

    fn from_ast(ast: Result<Vec<Var>, std::io::Error>) -> Self::Output;

    fn from_source(source: &str) -> Self::Output {
        let parser = parser::CodeParser::default();
        let source = source
            .replace("\r", "")
            .replace("\n", "");
        println!("Compiling source: \n{}", source);
        let ast = parser.parse(&source);
        println!("{:?}", ast);
        Self::from_ast(ast)
    }
}

pub struct Interpreter;

impl Compile for Interpreter {
    type Output = Result<i32, std::io::Error>;

    fn from_ast(ast: Result<Vec<Var>, std::io::Error>) -> Self::Output {
        let mut ret = 0i32;
        let evaluator = parser::Eval::default();
        match ast {
            Ok(ast) => {
                for var in ast {
                    ret += evaluator.eval(&var.expr);
                }
                Ok(ret)
            }
            Err(e) => {
                Err(std::io::Error::new(ErrorKind::InvalidInput, e.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    #[allow(unused_imports)]
    use crate::interpreter::{Interpreter, Compile};

    #[test]
    fn adding_two_numbers() {
        assert_eq!(Interpreter::from_source("main{let s = (1 + 2);}").unwrap(), 3);
    }

    #[test]
    fn subtracting_two_numbers() {
        assert_eq!(Interpreter::from_source("main{let s = (3 - 1);}").unwrap(), 2);
    }

    #[test]
    fn increasing_a_number() {
        assert_eq!(Interpreter::from_source("main{2++;}").unwrap(), 3);
    }

    #[test]
    fn decreasing_a_number() {
        assert_eq!(Interpreter::from_source("main{3--;}").unwrap(), 2);
    }

    #[test]
    fn multiplying_two_numbers() {
        assert_eq!(Interpreter::from_source("main{let s = (3*2);}").unwrap(), 6);
    }

    #[test]
    fn dividing_two_numbers() {
        assert_eq!(Interpreter::from_source("main {let s = (6/2);}").unwrap(), 3);
    }

    #[test]
    fn test_interpreter_grouping_priority() {
        let code1 = "main {
        let s = (3+(5*4));
        }";
        let code2 = "
        main {
            let s = ((3+5)*4);
        }
      ";
        assert_ne!(Interpreter::from_source(code1).unwrap(), 32);
        assert_eq!(Interpreter::from_source(code1).unwrap(), 23);
        assert_ne!(Interpreter::from_source(code2).unwrap(), 23);
        assert_eq!(Interpreter::from_source(code2).unwrap(), 32);
    }

    #[test]
    fn creating_a_variable() {
        assert_eq!(Interpreter::from_source("main {\
        let s = 2;
        }").unwrap(), 2);
    }

    #[test]
    fn requiring_a_main() {
        assert!(Interpreter::from_source("let s = 2;").is_err());
        assert!(Interpreter::from_source("main {let s = 2;}").is_ok());
        assert_eq!(Interpreter::from_source("main {
       let s = 2;
       }").unwrap(), 2);
    }

    #[test]
    fn reading_from_rs_file() {
        let file = std::fs::read_to_string("./static/example.rs").unwrap();
        assert_eq!(Interpreter::from_source(&file).unwrap(), 5);
    }
}
