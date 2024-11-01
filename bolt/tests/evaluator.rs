#![allow(dead_code, unused_imports)]
use std::{any::Any, rc::Rc};

use bolt::{
    error::EvaluatorError,
    evaluator::{
        self,
        environment::Environment,
        evaluator::{evaluate_statement, Evaluator},
    },
    object::object::{BooleanObj, Interger, Null, Object, Return},
    parser::{
        ast::{Identifier, Statement},
        parser::Parser,
    },
};

#[test]
fn test_eval() {
    let a = Interger { value: 10.0 };
    let input = "10;";
    let evaluator = Evaluator::new(input, "test", None, false, None, "");
    let evaluated = evaluator.eval().unwrap();
    match evaluated {
        Ok(eval) => {
            assert_eq!(format!("{:?}", eval), format!("{:?}", a));
        }
        Err(e) => {
            panic!("{:?}", e);
        }
    }
}

#[test]
fn test_integer() {
    //The output of this is same as input 10
    let input = "10";
    let evaluator = Evaluator::new(input, "test", None, false, None, "");
    let evaluated = evaluator.eval().unwrap();

    match evaluated {
        Ok(eval) => {
            assert_eq!(eval.inspect(), input.to_string());
        }
        Err(e) => {
            panic!("{:?}", e);
        }
    }
}

#[test]
fn test_boolean() {
    //The output of this is same as input true
    let input = "true";
    let evaluator = Evaluator::new(input, "test", None, false, None, "");
    let evaluated = evaluator.eval().unwrap();
    match evaluated {
        Ok(eval) => {
            assert_eq!(eval.inspect(), input.to_string());
        }
        Err(e) => {
            panic!("{:?}", e);
        }
    }
    let input = "false";
    let evaluator = Evaluator::new(input, "test", None, false, None, "");
    let evaluated = evaluator.eval().unwrap();
    match evaluated {
        Ok(eval) => {
            assert_eq!(eval.inspect(), input.to_string());
        }
        Err(e) => {
            panic!("{:?}", e);
        }
    }
}

#[test]
fn test_null() {
    //The output of this is same as input null
    let input = "null";
    let evaluator = Evaluator::new(input, "test", None, false, None, "");
    let evaluated = evaluator.eval().unwrap();

    match evaluated {
        Ok(eval) => {
            assert_eq!(eval.inspect(), input.to_string());
        }
        Err(e) => {
            panic!("{:?}", e);
        }
    }
}

#[test]
fn test_bool_prefix_evaluation() {
    let mut input = "!false";
    let evaluator = Evaluator::new(input, "test", None, false, None, "");
    let evaluated = evaluator.eval().unwrap();

    match evaluated {
        Ok(eval) => {
            assert_eq!(eval.inspect().as_str(), "true");
        }
        Err(e) => {
            panic!("{:?}", e);
        }
    }

    input = "!true";
    let evaluator = Evaluator::new(input, "test", None, false, None, "");
    let evaluated = evaluator.eval().unwrap();
    match evaluated {
        Ok(eval) => {
            assert_eq!(eval.inspect().as_str(), "false");
        }
        Err(e) => {
            panic!("{:?}", e);
        }
    }

    input = "!null";
    let evaluator = Evaluator::new(input, "Test", None, false, None, "");
    let evaluated = evaluator.eval().unwrap();
    match evaluated {
        Ok(eval) => {
            assert_eq!(eval.inspect().as_str(), "true");
        }
        Err(e) => {
            panic!("{:?}", e);
        }
    }
}

#[test]
fn test_minus_prefix_evaluation() {
    let input = "-5";
    let evaluator = Evaluator::new(input, "test", None, false, None, "");
    let evaluated = evaluator.eval().unwrap();

    match evaluated {
        Ok(eval) => {
            let value_any = eval.as_any();
            if let Some(int) = value_any.downcast_ref::<Interger>() {
                assert_eq!(int.value, -5.0);
            } else {
                panic!("Error Downcasting");
            }
        }
        Err(e) => {
            panic!("{:?}", e);
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
        let evaluator = Evaluator::new(tests[i], "test", None, false, None, "");
        let evaluated = evaluator.eval().unwrap();

        match evaluated {
            Ok(eval) => {
                let value_any = eval.as_any();
                if let Some(int) = value_any.downcast_ref::<Interger>() {
                    assert_eq!(int.value, expected_results[i]);
                } else {
                    panic!("Error Downcasting");
                }
            }
            Err(e) => {
                panic!("{:?}", e);
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
        let evaluator = Evaluator::new(tests[i], "test", None, false, None, "");
        let evaluated = evaluator.eval().unwrap();

        match evaluated {
            Ok(eval) => {
                let value_any = eval.as_any();

                if let Some(int) = value_any.downcast_ref::<BooleanObj>() {
                    assert_eq!(int.value, expected_results[i]);
                } else {
                    panic!("Error Downcasting");
                }
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
}

#[test]
fn test_conditional_evaluation() {
    let tests = [
        "if (true) { 10 };",
        "if (false) { 10 }; else { 1 }",
        "if (1) { 10 } else { 1 }",
        "if (1 > 2) { 10 } else { 20 }",
        "if (1 < 2) { 20 }",
        "if (1 < 2) { 10 } else { 20 }",
    ];
    let expected_results = vec![10.0, 1.0, 10.0, 20.0, 20.0, 10.0];
    let size = tests.len();
    for i in 0..size {
        let evaluator = Evaluator::new(tests[i], "test", None, false, None, "");
        let evaluated = evaluator.eval().unwrap();

        match evaluated {
            Ok(eval) => {
                let value_any = eval.as_any();

                if let Some(int) = value_any.downcast_ref::<Interger>() {
                    assert_eq!(int.value, expected_results[i]);
                } else {
                    panic!("Error Downcasting {:?}", eval);
                }
            }
            Err(e) => {
                panic!("At Test No - {} - {:?}", i, e);
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
        let evaluator = Evaluator::new(tests[i], "test", None, false, None, "");
        let evaluated = evaluator.eval().unwrap();

        match evaluated {
            Ok(eval) => {
                let value_any = eval.as_any();
                if let Some(val) = value_any.downcast_ref::<Null>() {
                    assert_eq!(val.inspect(), String::from("null"));
                } else {
                    panic!("Error Downcasting");
                }
            }
            Err(e) => {
                panic!("{:?}", e);
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
        let evaluator = Evaluator::new(tests[i], "test", None, false, None, "");
        let evaluated = evaluator.eval().unwrap();

        match evaluated {
            Ok(eval) => {
                let value_any = eval.as_any();

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
            Err(e) => {
                panic!("At Test No - {} - {:?}", i, e);
            }
        }
    }
}

#[test]
fn test_environment_evaluation() {
    let tests = [
        "let a = 10; a;",
        "let a = 5 * 5; a;",
        "let a = 5; let b = a; b;",
        "let a = 5; let b = a; let c = a + b + 5; c;",
    ];
    let expected_results = vec![10.0, 25.0, 5.0, 15.0];
    let size = tests.len();
    for i in 0..size {
        let evaluator = Evaluator::new(tests[i], "test", None, false, None, "");
        let evaluated = evaluator.eval().unwrap();

        match evaluated {
            Ok(eval) => {
                let value_any = eval.as_any();
                if let Some(val) = value_any.downcast_ref::<Interger>() {
                    assert_eq!(val.value, expected_results[i]);
                } else {
                    panic!("Error Downcasting");
                }
            }
            Err(e) => {
                panic!("At Test No - {} - {:?}", i, e);
            }
        }
    }
}

#[test]
fn test_functional_call_evaluation() {
    let tests = [
        "let a = fn(x){x;}; a(10);",
        "let a = fn(x){let c = x + 10; c;} a(10);",
        "let a = fn(x, y){let c = x + y; c;} a(10, 20);",
    ];
    let expected_results = vec![10.0, 20.0, 30.0];
    let size = tests.len();
    for i in 0..size {
        let evaluator = Evaluator::new(tests[i], "test", None, false, None, "");
        let evaluated = evaluator.eval().unwrap();

        match evaluated {
            Ok(eval) => {
                println!("Val - {:?}", eval);
                let value_any = eval.as_any();
                if let Some(val) = value_any.downcast_ref::<Interger>() {
                    assert_eq!(val.value, expected_results[i]);
                } else {
                    panic!("Error Downcasting");
                }
            }
            Err(e) => {
                panic!("At Test No - {} - {:?}", i, e);
            }
        }
    }
}
