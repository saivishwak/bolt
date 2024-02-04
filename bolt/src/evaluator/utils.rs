use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    error::{BoltError, EvaluatorError},
    object::{
        object::{BooleanObj, Function, Interger, Null, Object, Return},
        types::ObjectType,
    },
    parser::ast::{
        BlockStatement, Expression, Identifier, IfExpression, LetStatement, ReturnStatement,
        Statement,
    },
};

use super::{
    constants::{FALSE, NULL, TRUE},
    environment::{new_enclosed_environment, Environment},
    evaluator::{evaluate_expression, evaluate_statement},
};

pub fn evaluate_prefix_expression(
    operator: String,
    right: Rc<Box<dyn Object>>,
) -> Result<Rc<Box<dyn Object>>, EvaluatorError> {
    match operator.as_str() {
        "!" => {
            if right.get_type() == ObjectType::BOOLEAN {
                if right.inspect().as_str() == "true" {
                    return Ok(Rc::new(Box::new(FALSE)));
                } else {
                    return Ok(Rc::new(Box::new(TRUE)));
                }
            } else if right.get_type() == ObjectType::NULL {
                return Ok(Rc::new(Box::new(TRUE)));
            } else {
                return Err(EvaluatorError::new(
                    String::from("Only Boolean and NULL are allowed for ! prefix"),
                    None,
                    None,
                ));
            }
        }
        "-" => {
            let value_any = right.as_any();
            if let Some(int) = value_any.downcast_ref::<Interger>() {
                let new_float = -1.0 * int.value;
                return Ok(Rc::new(Box::new(Interger { value: new_float })));
            }
            return Err(EvaluatorError::new(
                String::from("Error Downcasting integer"),
                None,
                None,
            ));
        }
        _ => {
            return Err(EvaluatorError::new(
                String::from("Prefix that you are trying is not allowed"),
                None,
                None,
            ));
        }
    }
}

pub fn evaluate_binary_expression(
    operator: String,
    left: Rc<Box<dyn Object>>,
    right: Rc<Box<dyn Object>>,
) -> Result<Rc<Box<dyn Object>>, EvaluatorError> {
    let right_value_any = right.as_any();
    let left_value_any = left.as_any();
    if left.get_type() == ObjectType::INTERGER && right.get_type() == ObjectType::INTERGER {
        let right_val: &Interger;
        let left_value: &Interger;
        if let Some(int) = right_value_any.downcast_ref::<Interger>() {
            right_val = int;
        } else {
            return Err(EvaluatorError::new(
                String::from("Error Downcasting Integer"),
                None,
                None,
            ));
        }
        if let Some(int) = left_value_any.downcast_ref::<Interger>() {
            left_value = int;
        } else {
            return Err(EvaluatorError::new(
                String::from("Error Downcasting Integer"),
                None,
                None,
            ));
        }
        match operator.as_str() {
            "+" => {
                let new_value = left_value.value + right_val.value;
                return Ok(Rc::new(Box::new(Interger { value: new_value })));
            }
            "-" => {
                let new_value = left_value.value - right_val.value;
                return Ok(Rc::new(Box::new(Interger { value: new_value })));
            }
            "*" => {
                let new_value = left_value.value * right_val.value;
                return Ok(Rc::new(Box::new(Interger { value: new_value })));
            }
            "/" => {
                let new_value = left_value.value / right_val.value;
                return Ok(Rc::new(Box::new(Interger { value: new_value })));
            }
            "<" => {
                let new_value = left_value.value < right_val.value;
                return Ok(Rc::new(Box::new(BooleanObj { value: new_value })));
            }
            ">" => {
                let new_value = left_value.value > right_val.value;
                return Ok(Rc::new(Box::new(BooleanObj { value: new_value })));
            }
            "==" => {
                let new_value = left_value.value == right_val.value;
                return Ok(Rc::new(Box::new(BooleanObj { value: new_value })));
            }
            "!=" => {
                let new_value = left_value.value != right_val.value;
                return Ok(Rc::new(Box::new(BooleanObj { value: new_value })));
            }
            ">=" => {
                let new_value = left_value.value >= right_val.value;
                return Ok(Rc::new(Box::new(BooleanObj { value: new_value })));
            }
            "<=" => {
                let new_value = left_value.value <= right_val.value;
                return Ok(Rc::new(Box::new(BooleanObj { value: new_value })));
            }
            _ => {
                return Err(EvaluatorError::new(
                    String::from("Binary Expression oprerator match failed"),
                    None,
                    None,
                ));
            }
        }
    } else if left.get_type() == ObjectType::BOOLEAN && right.get_type() == ObjectType::BOOLEAN {
        let right_val: &BooleanObj;
        let left_value: &BooleanObj;
        if let Some(bool) = right_value_any.downcast_ref::<BooleanObj>() {
            right_val = bool;
        } else {
            return Err(EvaluatorError::new(
                String::from("Downcasting boolean failed"),
                None,
                None,
            ));
        }
        if let Some(bool) = left_value_any.downcast_ref::<BooleanObj>() {
            left_value = bool;
        } else {
            return Err(EvaluatorError::new(
                String::from("Downcasting boolean failed"),
                None,
                None,
            ));
        }
        match operator.as_str() {
            "<" => {
                let new_value = left_value.value < right_val.value;
                return Ok(Rc::new(Box::new(BooleanObj { value: new_value })));
            }
            ">" => {
                let new_value = left_value.value > right_val.value;
                return Ok(Rc::new(Box::new(BooleanObj { value: new_value })));
            }
            "==" => {
                let new_value = left_value.value == right_val.value;
                return Ok(Rc::new(Box::new(BooleanObj { value: new_value })));
            }
            "!=" => {
                let new_value = left_value.value != right_val.value;
                return Ok(Rc::new(Box::new(BooleanObj { value: new_value })));
            }
            ">=" => {
                let new_value = left_value.value >= right_val.value;
                return Ok(Rc::new(Box::new(BooleanObj { value: new_value })));
            }
            "<=" => {
                let new_value = left_value.value <= right_val.value;
                return Ok(Rc::new(Box::new(BooleanObj { value: new_value })));
            }
            _ => {
                return Err(EvaluatorError::new(
                    String::from("Binary Expression oprerator match failed for boolean"),
                    None,
                    None,
                ));
            }
        }
    } else {
        // Cases like 1 + true or 1 > true , true + 1 true > 1 are errored
        return Err(EvaluatorError::new(
            String::from("Invalid binary left and right operands"),
            None,
            None,
        ));
    }
}

pub fn is_truthy(condition: Rc<Box<dyn Object>>) -> bool {
    let value_any = condition.as_any();
    if let Some(value) = value_any.downcast_ref::<BooleanObj>() {
        return value.value;
    } else if let Some(value) = value_any.downcast_ref::<Interger>() {
        if value.value == 0.0 {
            return false;
        }
        return true;
    } else if let Some(_value) = value_any.downcast_ref::<Null>() {
        return false;
    }
    return false;
}

pub fn evaluate_block_statements(
    statements: &Vec<Box<dyn Statement>>,
    environment: Rc<RefCell<Environment>>,
) -> Result<Rc<Box<dyn Object>>, EvaluatorError> {
    let mut result: Option<Result<Rc<Box<dyn Object>>, EvaluatorError>> = None;
    for statement in statements {
        result = Some(evaluate_statement(statement, environment.clone()));
        if let Some(res) = &result {
            let value_any = res.as_ref().unwrap().as_any();
            if let Some(_v) = value_any.downcast_ref::<Return>() {
                return result.unwrap();
            }
        }
    }
    if let Some(res) = result {
        return res;
    } else {
        return Err(EvaluatorError::new(
            String::from("Error evaluating block statement"),
            None,
            None,
        ));
    }
}

pub fn evaluate_block_statement(
    block_statement: &Box<BlockStatement>,
    environment: Rc<RefCell<Environment>>,
) -> Result<Rc<Box<dyn Object>>, EvaluatorError> {
    let statements = &block_statement.statements;
    return evaluate_block_statements(statements, environment);
}

pub fn evaluate_block_statement_ref(
    block_statement: &BlockStatement,
    environment: Rc<RefCell<Environment>>,
) -> Result<Rc<Box<dyn Object>>, EvaluatorError> {
    let statements = &block_statement.statements;
    return evaluate_block_statements(statements, environment);
}

pub fn evaluate_condition_expression(
    if_expression: &IfExpression,
    environment: Rc<RefCell<Environment>>,
) -> Result<Rc<Box<dyn Object>>, EvaluatorError> {
    let condition_eval = evaluate_expression(&if_expression.condition, environment.clone())?;
    let truthy = is_truthy(condition_eval);
    if truthy {
        let consequence = &if_expression.consequence;
        return evaluate_block_statement(consequence, environment.clone());
    } else {
        match &if_expression.alternate.as_ref() {
            Some(alternate) => {
                return evaluate_block_statement(alternate, environment.clone());
            }
            None => {
                return Ok(Rc::new(Box::new(NULL)));
            }
        }
    }
}

pub fn evaluate_return_statement(
    return_statement: &ReturnStatement,
    environment: Rc<RefCell<Environment>>,
) -> Result<Rc<Box<dyn Object>>, EvaluatorError> {
    match evaluate_expression(&return_statement.value, environment) {
        Ok(value) => {
            return Ok(Rc::new(Box::new(Return { value: value })));
        }
        Err(e) => return Err(e),
    }
}

pub fn evaluate_let_statement(
    let_statement: &LetStatement,
    environment: Rc<RefCell<Environment>>,
) -> Result<Rc<Box<dyn Object>>, EvaluatorError> {
    match evaluate_expression(&let_statement.value, environment.clone()) {
        Ok(value) => match environment.try_borrow_mut() {
            Ok(mut mutable_ref) => {
                let ident = let_statement.identifier.value.clone();
                match mutable_ref.set(ident.clone(), value.clone()) {
                    Some(_v) => {
                        return Ok(value.clone());
                    }
                    None => {
                        return Err(EvaluatorError::new(
                            String::from("Error Setting environment value"),
                            None,
                            None,
                        ));
                    }
                }
            }
            Err(e) => {
                return Err(EvaluatorError::new(e.to_string(), None, None));
            }
        },
        Err(e) => return Err(e),
    }
}

pub fn evaluate_identifier(
    identifier: &Identifier,
    environment: Rc<RefCell<Environment>>,
) -> Result<Rc<Box<dyn Object>>, EvaluatorError> {
    let ident = identifier.value.clone();
    match environment.try_borrow() {
        Ok(borrow_ref) => {
            let optional_value = borrow_ref.get(ident.clone());
            match optional_value {
                Some(value) => {
                    return Ok(value);
                }
                None => {
                    //Todo if not found in env should be assign NULL or Panic?
                    // return Ok(Rc::new(Box::new(NULL)));
                    return Err(EvaluatorError::new(
                        String::from(format!(
                            "Error getting environment variable {}",
                            ident.clone()
                        )),
                        None,
                        None,
                    ));
                }
            }
        }
        Err(e) => {
            return Err(EvaluatorError::new(e.to_string(), None, None));
        }
    }
}

pub fn extend_funtion_env(
    function: &Function,
    args: HashMap<String, Rc<Box<dyn Object>>>,
) -> Rc<RefCell<Environment>> {
    let env = new_enclosed_environment(function.env.clone());

    for param in args {
        env.borrow_mut().set(param.0, param.1.clone());
    }

    return env;
}

pub fn apply_function(
    function: Rc<Box<dyn Object>>,
    args: HashMap<String, Rc<Box<dyn Object>>>,
    _env: Rc<RefCell<Environment>>,
) -> Result<Rc<Box<dyn Object>>, EvaluatorError> {
    let value_any = function.as_any();
    if let Some(function_value) = value_any.downcast_ref::<Function>() {
        let extended_env = extend_funtion_env(function_value, args);
        let evaluated =
            evaluate_block_statement_ref(function_value.body.as_ref(), extended_env.clone());
        return evaluated;
    } else {
        return Err(EvaluatorError::new(
            "Error in evaluating function".to_string(),
            None,
            None,
        ));
    }
}

pub fn eval_arg_expression(
    args: Rc<Vec<Box<dyn Expression>>>,
    function: Rc<Box<dyn Object>>,
    env: Rc<RefCell<Environment>>,
) -> Result<HashMap<String, Rc<Box<dyn Object>>>, EvaluatorError> {
    let mut result: HashMap<String, Rc<Box<dyn Object>>> = HashMap::new();
    let value_any = function.as_any();
    if let Some(function_object) = value_any.downcast_ref::<Function>() {
        let params = function_object.parameters.clone();
        let length = params.len();
        if length != args.len() {
            return Err(EvaluatorError::new(
                "No of args in function mismatch".to_string(),
                None,
                None,
            ));
        }
        for i in 0..length {
            let param = params[i].value.clone();
            let evaluated = evaluate_expression(&args[i], env.clone())?;
            result.insert(param, evaluated);
        }

        return Ok(result);
    } else {
        return Err(EvaluatorError::new(
            "Error Downcasting function".to_string(),
            None,
            None,
        ));
    }
}
