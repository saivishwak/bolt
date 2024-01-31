#![allow(dead_code, unused_imports)]
use std::any::Any;

use bolt::{
    evaluator::evaluator::evaluate_statement,
    object::object::Interger,
    parser::{
        ast::{IntegerLiteral, Statement},
        parser::Parser,
    },
};

#[test]
fn test_eval() {
    let a = Interger { v: 10.0 };
    let input = "10;";
    let mut parser = Parser::new(&input);
    let p = parser.parse_program();
    match p {
        Ok(program) => {
            let stmts = program.stmts;
            for stmt in stmts {
                let e = evaluate_statement(&stmt).unwrap();
                assert_eq!(format!("{:?}", e), format!("{:?}", a));
            }
        }
        Err(e) => {
            print!("Error - {:?}", e.message);
        }
    }
}

#[test]
fn test_integer() {
    //The output of this is same as input 10
    let input = "10";
    let mut parser = Parser::new(&input);
    let p = parser.parse_program();
    match p {
        Ok(program) => {
            let stmts = program.stmts;
            for stmt in stmts {
                let e = evaluate_statement(&stmt).unwrap();
                assert_eq!(e.inspect(), input.to_string());
            }
        }
        Err(e) => {
            print!("Error - {:?}", e.message);
        }
    }
}

#[test]
fn test_boolean() {
    //The output of this is same as input true
    let input = "true";
    let mut parser = Parser::new(&input);
    let p = parser.parse_program();
    match p {
        Ok(program) => {
            let stmts = program.stmts;
            for stmt in stmts {
                let e = evaluate_statement(&stmt).unwrap();
                assert_eq!(e.inspect(), input.to_string());
            }
        }
        Err(e) => {
            print!("Error - {:?}", e.message);
        }
    }

    let input = "false";
    let mut parser = Parser::new(&input);
    let p = parser.parse_program();
    match p {
        Ok(program) => {
            let stmts = program.stmts;
            for stmt in stmts {
                let e = evaluate_statement(&stmt).unwrap();
                assert_eq!(e.inspect(), input.to_string());
            }
        }
        Err(e) => {
            print!("Error - {:?}", e.message);
        }
    }
}

#[test]
fn test_null() {
    //The output of this is same as input null
    let input = "null";
    let mut parser = Parser::new(&input);
    let p = parser.parse_program();
    match p {
        Ok(program) => {
            let stmts = program.stmts;
            for stmt in stmts {
                let e = evaluate_statement(&stmt).unwrap();
                assert_eq!(e.inspect(), input.to_string());
            }
        }
        Err(e) => {
            print!("Error - {:?}", e.message);
        }
    }
}

#[test]
fn test_bool_prefix_evaluation() {
    //The output of this is same as input null
    let input = "!false";
    let mut parser = Parser::new(&input);
    let p = parser.parse_program();
    match p {
        Ok(program) => {
            let stmts = program.stmts;
            for stmt in stmts {
                let e = evaluate_statement(&stmt).unwrap();
                assert_eq!(e.inspect().as_str(), "true");
            }
        }
        Err(e) => {
            print!("Error - {:?}", e.message);
        }
    }

    let input = "!true";
    let mut parser = Parser::new(&input);
    let p = parser.parse_program();
    match p {
        Ok(program) => {
            let stmts = program.stmts;
            for stmt in stmts {
                let e = evaluate_statement(&stmt).unwrap();
                assert_eq!(e.inspect().as_str(), "false");
            }
        }
        Err(e) => {
            print!("Error - {:?}", e.message);
        }
    }

    let input = "!null";
    let mut parser = Parser::new(&input);
    let p = parser.parse_program();
    match p {
        Ok(program) => {
            let stmts = program.stmts;
            for stmt in stmts {
                let e = evaluate_statement(&stmt).unwrap();
                assert_eq!(e.inspect().as_str(), "true");
            }
        }
        Err(e) => {
            print!("Error - {:?}", e.message);
        }
    }
}

#[test]
fn test_minus_prefix_evaluation() {
    //The output of this is same as input null
    let input = "-5";
    let mut parser = Parser::new(&input);
    let p = parser.parse_program();
    match p {
        Ok(program) => {
            let stmts = program.stmts;
            for stmt in stmts {
                let e = evaluate_statement(&stmt).unwrap();
                let value_any = e.as_any();
                if let Some(int) = value_any.downcast_ref::<Interger>() {
                    assert_eq!(int.v, -5.0);
                } else {
                    print!("Error Downcasting");
                }
            }
        }
        Err(e) => {
            print!("Error - {:?}", e.message);
        }
    }
}
