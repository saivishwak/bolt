use std::rc::Rc;

use crate::{
    error::{BoltError, EvaluatorError},
    object::object::{Interger, Object},
    parser::{
        ast::{
            BinaryExpression, BlockStatement, Boolean, Expression, ExpressionStatement, Identifier,
            IfExpression, IntegerLiteral, LetStatement, NullLiteral, PrefixExpression,
            ReturnStatement, Statement,
        },
        parser::Parser,
    },
};

use super::{
    constants::{FALSE, NULL, TRUE},
    environment::Environment,
    utils::{
        evaluate_binary_expression, evaluate_block_statement_ref, evaluate_condition_expression,
        evaluate_identifier, evaluate_let_statement, evaluate_prefix_expression,
        evaluate_return_statement,
    },
};

pub fn eval(
    source: String,
    environment: &mut Environment,
) -> Option<Result<Rc<Box<dyn Object>>, EvaluatorError>> {
    let mut parser = Parser::new(&source);
    let mut evaluated_result: Option<Result<Rc<Box<dyn Object>>, EvaluatorError>> = None;
    match parser.parse_program() {
        Ok(program) => {
            for stmt in program.stmts {
                match evaluate_statement(&stmt, environment) {
                    Ok(eval) => {
                        evaluated_result = Some(Ok(eval));
                    }
                    Err(e) => {
                        return Some(Err(EvaluatorError::new(
                            e.get_message(),
                            Some(e.get_type()),
                            None,
                        )));
                    }
                }
            }
        }
        Err(e) => {
            return Some(Err(EvaluatorError::new(
                e.get_message(),
                Some(e.get_type()),
                None,
            )));
        }
    }
    return evaluated_result;
}

pub fn evaluate_expression(
    expression: &Box<dyn Expression>,
    environment: &mut Environment,
) -> Result<Rc<Box<dyn Object>>, EvaluatorError> {
    let value_any = expression.as_any();
    if let Some(int) = value_any.downcast_ref::<IntegerLiteral>() {
        return Ok(Rc::new(Box::new(Interger { value: int.value })));
    } else if let Some(boolean) = value_any.downcast_ref::<Boolean>() {
        if boolean.value == true {
            return Ok(Rc::new(Box::new(TRUE)));
        } else {
            return Ok(Rc::new(Box::new(FALSE)));
        }
    } else if let Some(prefix) = value_any.downcast_ref::<PrefixExpression>() {
        let right = evaluate_expression(&prefix.right, environment);
        return evaluate_prefix_expression(prefix.operator.clone(), right.unwrap());
    } else if let Some(binary) = value_any.downcast_ref::<BinaryExpression>() {
        let left = evaluate_expression(&binary.left, environment);
        let right: Result<Rc<Box<dyn Object>>, EvaluatorError> =
            evaluate_expression(&binary.right, environment);
        return evaluate_binary_expression(binary.operator.clone(), left.unwrap(), right.unwrap());
    } else if let Some(if_expression) = value_any.downcast_ref::<IfExpression>() {
        return evaluate_condition_expression(if_expression, environment);
    } else if let Some(ident) = value_any.downcast_ref::<Identifier>() {
        return evaluate_identifier(ident, environment);
    } else if let Some(_null) = value_any.downcast_ref::<NullLiteral>() {
        return Ok(Rc::new(Box::new(NULL)));
    } else {
        return Err(EvaluatorError::new(
            String::from("Expression found found for eval"),
            None,
            None,
        ));
    }
}

pub fn evaluate_statement(
    statement: &Box<dyn Statement>,
    environment: &mut Environment,
) -> Result<Rc<Box<dyn Object>>, EvaluatorError> {
    let value_any = statement.as_any();
    if let Some(expr) = value_any.downcast_ref::<ExpressionStatement>() {
        return evaluate_expression(&expr.value, environment);
    } else if let Some(block_statement) = value_any.downcast_ref::<BlockStatement>() {
        return evaluate_block_statement_ref(block_statement, environment);
    } else if let Some(return_statement) = value_any.downcast_ref::<ReturnStatement>() {
        return evaluate_return_statement(return_statement, environment);
    } else if let Some(let_statement) = value_any.downcast_ref::<LetStatement>() {
        return evaluate_let_statement(let_statement, environment);
    } else {
        return Err(EvaluatorError::new(
            String::from("Requested Statement type not found"),
            None,
            None,
        ));
    }
}
