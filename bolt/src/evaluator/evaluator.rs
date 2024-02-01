use crate::{
    object::object::{Interger, Object},
    parser::ast::{
        BinaryExpression, BlockStatement, Boolean, Expression, ExpressionStatement, IfExpression,
        IntegerLiteral, NullLiteral, PrefixExpression, Statement,
    },
};

use super::{
    constants::{FALSE, NULL, TRUE},
    utils::{evaluate_binary_expression, evaluate_prefix_expression, is_truthy},
};

pub fn evaluate_expression(expression: &Box<dyn Expression>) -> Result<Box<dyn Object>, ()> {
    let value_any = expression.as_any();
    if let Some(int) = value_any.downcast_ref::<IntegerLiteral>() {
        return Ok(Box::new(Interger { v: int.value }));
    } else if let Some(boolean) = value_any.downcast_ref::<Boolean>() {
        if boolean.value == true {
            return Ok(Box::new(TRUE));
        } else {
            return Ok(Box::new(FALSE));
        }
    } else if let Some(prefix) = value_any.downcast_ref::<PrefixExpression>() {
        let right = evaluate_expression(&prefix.right);
        return evaluate_prefix_expression(prefix.operator.clone(), right.unwrap());
    } else if let Some(binary) = value_any.downcast_ref::<BinaryExpression>() {
        let left = evaluate_expression(&binary.left);
        let right = evaluate_expression(&binary.right);
        return evaluate_binary_expression(binary.operator.clone(), left.unwrap(), right.unwrap());
    } else if let Some(if_expr) = value_any.downcast_ref::<IfExpression>() {
        let condition_eval = evaluate_expression(&if_expr.condition).unwrap();
        let truthy = is_truthy(condition_eval);
        if truthy {
            let consequence = &if_expr.consequence;
            return evaluate_block_statement(consequence);
        } else {
            match &if_expr.alternate.as_ref() {
                Some(alternate) => {
                    return evaluate_block_statement(alternate);
                }
                None => {
                    return Ok(Box::new(NULL));
                }
            }
        }
    } else if let Some(_null) = value_any.downcast_ref::<NullLiteral>() {
        return Ok(Box::new(NULL));
    } else {
        return Err(());
    }
}

pub fn evaluate_block_statement(
    block_statement: &Box<BlockStatement>,
) -> Result<Box<dyn Object>, ()> {
    let statements = &block_statement.statements;
    let mut result: Option<Result<Box<dyn Object>, ()>> = None;
    for statement in statements {
        result = Some(evaluate_statement(statement));
    }
    if let Some(res) = result {
        return res;
    } else {
        return Err(());
    }
}

pub fn evaluate_block_statement_ref(
    block_statement: &BlockStatement,
) -> Result<Box<dyn Object>, ()> {
    let statements = &block_statement.statements;
    let mut result: Option<Result<Box<dyn Object>, ()>> = None;
    for statement in statements {
        result = Some(evaluate_statement(statement));
    }
    if let Some(res) = result {
        return res;
    } else {
        return Err(());
    }
}

pub fn evaluate_statement(statement: &Box<dyn Statement>) -> Result<Box<dyn Object>, ()> {
    let value_any = statement.as_any();
    if let Some(expr) = value_any.downcast_ref::<ExpressionStatement>() {
        return evaluate_expression(&expr.value);
    } else if let Some(block_statement) = value_any.downcast_ref::<BlockStatement>() {
        return evaluate_block_statement_ref(block_statement);
    } else {
        return Err(());
    }
}
