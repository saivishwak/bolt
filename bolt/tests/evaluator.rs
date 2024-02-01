#![allow(dead_code, unused_imports)]
use std::any::Any;

use bolt::{
    evaluator::evaluator::evaluate_statement,
    object::object::{BooleanObj, Interger, Null, Object, Return},
    parser::{ast::Statement, parser::Parser},
};

#[test]
fn test_eval() {
    let a = Interger { value: 10.0 };
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
            panic!("Error - {:?}", e.message);
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
            panic!("Error - {:?}", e.message);
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
            panic!("Error - {:?}", e.message);
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
            panic!("Error - {:?}", e.message);
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
            panic!("Error - {:?}", e.message);
        }
    }
}

#[test]
fn test_bool_prefix_evaluation() {
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
            panic!("Error - {:?}", e.message);
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
            panic!("Error - {:?}", e.message);
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
            panic!("Error - {:?}", e.message);
        }
    }
}

#[test]
fn test_minus_prefix_evaluation() {
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
                    assert_eq!(int.value, -5.0);
                } else {
                    panic!("Error Downcasting");
                }
            }
        }
        Err(e) => {
            panic!("Error - {:?}", e.message);
        }
    }
}

#[test]
fn test_number_binary_evaluation() {
    let tests = [
        "5",
        "-5",
        "1 + 2",
        "2 * 2",
        "4 / 2",
        "2 + 2 * 2",
        "5 * 0",
        "( 2 * 2 + 2)",
        "3 * (3 * 3) + 10",
        "20 + 2 * -10",
        "-50 + 100 + -50",
        "(5 + 10 * 2 + 15 / 3) * 2 + -10",
        "5 + 2 * 10",
        "2 * 2 * 2 * 2 * 2",
    ];
    let expected_results = vec![
        5.0, -5.0, 3.0, 4.0, 2.0, 6.0, 0.0, 6.0, 37.0, 0.0, 0.0, 50.0, 25.0, 32.0,
    ];
    let size = tests.len();
    for i in 0..size {
        let mut parser = Parser::new(tests[i]);
        let p = parser.parse_program();
        match p {
            Ok(program) => {
                let stmts = program.stmts;
                for stmt in stmts {
                    let e = evaluate_statement(&stmt).unwrap();
                    let value_any = e.as_any();
                    if let Some(int) = value_any.downcast_ref::<Interger>() {
                        assert_eq!(int.value, expected_results[i]);
                    } else {
                        panic!("Error Downcasting");
                    }
                }
            }
            Err(e) => {
                panic!("Error - {:?}", e.message);
            }
        }
    }
}

#[test]
fn test_boolean_binary_evaluation() {
    let tests = [
        "1 < 2",
        "1 > 2",
        "1 < 1",
        "1 > 1",
        "1 == 1",
        "1 != 1",
        "1 == 2",
        "1 != 2",
        "1 >= 1",
        "3 <= 2",
        "true == true",
        "false == false",
        "true == false",
        "true != false",
        "false != true",
    ];
    let expected_results = vec![
        true, false, false, false, true, false, false, true, true, false, true, true, false, true,
        true,
    ];
    let size = tests.len();
    for i in 0..size {
        let mut parser = Parser::new(tests[i]);
        let p = parser.parse_program();
        match p {
            Ok(program) => {
                let stmts = program.stmts;
                for stmt in stmts {
                    let e = evaluate_statement(&stmt).unwrap();
                    let value_any = e.as_any();
                    if let Some(int) = value_any.downcast_ref::<BooleanObj>() {
                        assert_eq!(int.value, expected_results[i]);
                    } else {
                        panic!("Error Downcasting");
                    }
                }
            }
            Err(e) => {
                panic!("Error - {:?}", e.message);
            }
        }
    }
}

#[test]
fn test_conditional_evaluation() {
    let tests = [
        "if (true) { 10 }",
        "if (false) { 10 } else { 1 }",
        "if (1) { 10 } else { 1 }",
        "if (1 > 2) { 10 } else { 20 }",
        "if (1 < 2) { 20 }",
        "if (1 < 2) { 10 } else { 20 }",
    ];
    let expected_results = vec![10.0, 1.0, 10.0, 20.0, 20.0, 10.0];
    let size = tests.len();
    for i in 0..size {
        let mut parser = Parser::new(tests[i]);
        let p = parser.parse_program();
        match p {
            Ok(program) => {
                let stmts = program.stmts;
                for stmt in stmts {
                    let e = evaluate_statement(&stmt).unwrap();
                    let value_any = e.as_any();
                    if let Some(int) = value_any.downcast_ref::<Interger>() {
                        assert_eq!(int.value, expected_results[i]);
                    } else {
                        panic!("Error Downcasting {:?}", e);
                    }
                }
            }
            Err(e) => {
                panic!("Error - {:?}", e.message);
            }
        }
    }
}

#[test]
fn test_conditional_evaluation_nil() {
    let tests = [
        "if (false) { 10 }",
        "if (true) { null }",
        "if (1 > 2) { 10 }",
    ];
    let size = tests.len();
    for i in 0..size {
        let mut parser = Parser::new(tests[i]);
        let p = parser.parse_program();
        match p {
            Ok(program) => {
                let stmts = program.stmts;
                for stmt in stmts {
                    let e = evaluate_statement(&stmt).unwrap();
                    let value_any = e.as_any();
                    if let Some(val) = value_any.downcast_ref::<Null>() {
                        assert_eq!(val.inspect(), String::from("null"));
                    } else {
                        panic!("Error Downcasting");
                    }
                }
            }
            Err(e) => {
                panic!("Error - {:?}", e.message);
            }
        }
    }
}

#[test]
fn test_return_evaluation() {
    let tests = [
        "if (true) { return 10 }",
        "if (true) { if (1) {return 10}; return 100 }",
        "if (false) {return 1}else {return 10}",
        "if (true) { if (true) { if (false) {return 1} else {return 10}; return 20}; return 30}",
    ];
    let expected_results = vec![10.0, 10.0, 10.0, 10.0];
    let size = tests.len();
    for i in 0..size {
        let mut parser = Parser::new(tests[i]);
        let p = parser.parse_program();
        match p {
            Ok(program) => {
                let stmts = program.stmts;
                for stmt in stmts {
                    let e = evaluate_statement(&stmt).unwrap();
                    let value_any = e.as_any();
                    if let Some(val) = value_any.downcast_ref::<Return>() {
                        let value_any = val.value.as_any();
                        if let Some(val) = value_any.downcast_ref::<Interger>() {
                            assert_eq!(val.value, expected_results[i]);
                        } else {
                            panic!("Error Integer Downcasting");
                        }
                    } else {
                        panic!("Error Downcasting");
                    }
                }
            }
            Err(e) => {
                print!("Error - {:?}", e.message);
            }
        }
    }
}
