use crate::{
    object::{
        object::{BooleanObj, Interger, Null, Object, Return},
        types::ObjectType,
    },
    parser::ast::{BlockStatement, IfExpression, ReturnStatement, Statement},
};

use super::{
    constants::{FALSE, NULL, TRUE},
    evaluator::{evaluate_expression, evaluate_statement},
};

pub fn evaluate_prefix_expression(
    operator: String,
    right: Box<dyn Object>,
) -> Result<Box<dyn Object>, ()> {
    match operator.as_str() {
        "!" => {
            if right.get_type() == ObjectType::BOOLEAN {
                if right.inspect().as_str() == "true" {
                    return Ok(Box::new(FALSE));
                } else {
                    return Ok(Box::new(TRUE));
                }
            } else if right.get_type() == ObjectType::NULL {
                return Ok(Box::new(TRUE));
            } else {
                return Err(());
            }
        }
        "-" => {
            let value_any = right.as_any();
            if let Some(int) = value_any.downcast_ref::<Interger>() {
                let new_float = -1.0 * int.value;
                return Ok(Box::new(Interger { value: new_float }));
            }
            return Err(());
        }
        _ => {
            return Err(());
        }
    }
}

pub fn evaluate_binary_expression(
    operator: String,
    left: Box<dyn Object>,
    right: Box<dyn Object>,
) -> Result<Box<dyn Object>, ()> {
    let right_value_any = right.as_any();
    let left_value_any = left.as_any();
    if left.get_type() == ObjectType::INTERGER && right.get_type() == ObjectType::INTERGER {
        let right_val: &Interger;
        let left_value: &Interger;
        if let Some(int) = right_value_any.downcast_ref::<Interger>() {
            right_val = int;
        } else {
            return Err(());
        }
        if let Some(int) = left_value_any.downcast_ref::<Interger>() {
            left_value = int;
        } else {
            return Err(());
        }
        match operator.as_str() {
            "+" => {
                let new_value = left_value.value + right_val.value;
                return Ok(Box::new(Interger { value: new_value }));
            }
            "-" => {
                let new_value = left_value.value - right_val.value;
                return Ok(Box::new(Interger { value: new_value }));
            }
            "*" => {
                let new_value = left_value.value * right_val.value;
                return Ok(Box::new(Interger { value: new_value }));
            }
            "/" => {
                let new_value = left_value.value / right_val.value;
                return Ok(Box::new(Interger { value: new_value }));
            }
            "<" => {
                let new_value = left_value.value < right_val.value;
                return Ok(Box::new(BooleanObj { value: new_value }));
            }
            ">" => {
                let new_value = left_value.value > right_val.value;
                return Ok(Box::new(BooleanObj { value: new_value }));
            }
            "==" => {
                let new_value = left_value.value == right_val.value;
                return Ok(Box::new(BooleanObj { value: new_value }));
            }
            "!=" => {
                let new_value = left_value.value != right_val.value;
                return Ok(Box::new(BooleanObj { value: new_value }));
            }
            ">=" => {
                let new_value = left_value.value >= right_val.value;
                return Ok(Box::new(BooleanObj { value: new_value }));
            }
            "<=" => {
                let new_value = left_value.value <= right_val.value;
                return Ok(Box::new(BooleanObj { value: new_value }));
            }
            _ => return Err(()),
        }
    } else if left.get_type() == ObjectType::BOOLEAN && right.get_type() == ObjectType::BOOLEAN {
        let right_val: &BooleanObj;
        let left_value: &BooleanObj;
        if let Some(bool) = right_value_any.downcast_ref::<BooleanObj>() {
            right_val = bool;
        } else {
            return Err(());
        }
        if let Some(bool) = left_value_any.downcast_ref::<BooleanObj>() {
            left_value = bool;
        } else {
            return Err(());
        }
        match operator.as_str() {
            "<" => {
                let new_value = left_value.value < right_val.value;
                return Ok(Box::new(BooleanObj { value: new_value }));
            }
            ">" => {
                let new_value = left_value.value > right_val.value;
                return Ok(Box::new(BooleanObj { value: new_value }));
            }
            "==" => {
                let new_value = left_value.value == right_val.value;
                return Ok(Box::new(BooleanObj { value: new_value }));
            }
            "!=" => {
                let new_value = left_value.value != right_val.value;
                return Ok(Box::new(BooleanObj { value: new_value }));
            }
            ">=" => {
                let new_value = left_value.value >= right_val.value;
                return Ok(Box::new(BooleanObj { value: new_value }));
            }
            "<=" => {
                let new_value = left_value.value <= right_val.value;
                return Ok(Box::new(BooleanObj { value: new_value }));
            }
            _ => return Err(()),
        }
    } else {
        // Cases like 1 + true or 1 > true , true + 1 true > 1 are errored
        return Err(());
    }
}

pub fn is_truthy(condition: Box<dyn Object>) -> bool {
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
) -> Result<Box<dyn Object>, ()> {
    let mut result: Option<Result<Box<dyn Object>, ()>> = None;
    for statement in statements {
        result = Some(evaluate_statement(statement));
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
        return Err(());
    }
}

pub fn evaluate_block_statement(
    block_statement: &Box<BlockStatement>,
) -> Result<Box<dyn Object>, ()> {
    let statements = &block_statement.statements;
    return evaluate_block_statements(statements);
}

pub fn evaluate_block_statement_ref(
    block_statement: &BlockStatement,
) -> Result<Box<dyn Object>, ()> {
    let statements = &block_statement.statements;
    return evaluate_block_statements(statements);
}

pub fn evaluate_condition_expression(if_expression: &IfExpression) -> Result<Box<dyn Object>, ()> {
    let condition_eval = evaluate_expression(&if_expression.condition).unwrap();
    let truthy = is_truthy(condition_eval);
    if truthy {
        let consequence = &if_expression.consequence;
        return evaluate_block_statement(consequence);
    } else {
        match &if_expression.alternate.as_ref() {
            Some(alternate) => {
                return evaluate_block_statement(alternate);
            }
            None => {
                return Ok(Box::new(NULL));
            }
        }
    }
}

pub fn evaluate_return_statement(
    return_statement: &ReturnStatement,
) -> Result<Box<dyn Object>, ()> {
    match evaluate_expression(&return_statement.value) {
        Ok(value) => {
            return Ok(Box::new(Return { value: value }));
        }
        Err(e) => return Err(e),
    }
}
