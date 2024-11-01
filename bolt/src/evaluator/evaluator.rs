use std::{cell::RefCell, rc::Rc};

use crate::{
    compiler::{Compiler, CompilerBackend, Factory},
    error::{BoltError, EvaluatorError},
    evaluator::utils::{apply_function, eval_arg_expression},
    object::object::{Function, Interger, Object},
    parser::{
        ast::{
            BinaryExpression, BlockStatement, Boolean, CallExpression, Expression,
            ExpressionStatement, FunctionLiteral, Identifier, IfExpression, IntegerLiteral,
            LetStatement, NullLiteral, PrefixExpression, ReturnStatement, Statement,
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

pub struct Evaluator {
    source: String,
    filename: String,
    environment: Rc<RefCell<Environment>>,
    jit: bool,
    target: String,
    backend: Option<CompilerBackend>,
}

impl Evaluator {
    pub fn new(
        source: &str,
        filename: &str,
        environment: Option<Rc<RefCell<Environment>>>,
        jit: bool,
        backend: Option<CompilerBackend>,
        target: &str,
    ) -> Self {
        let environment = environment.unwrap_or(Environment::new());
        Self {
            source: source.into(),
            environment: environment,
            jit,
            backend,
            target: target.into(),
            filename: filename.into(),
        }
    }

    fn eval_jit(&self) -> Option<Result<Rc<Box<dyn Object>>, EvaluatorError>> {
        let source = self.source.clone();
        let mut parser = Parser::new(&source);
        let evaluated_result: Option<Result<Rc<Box<dyn Object>>, EvaluatorError>> = None;
        match parser.parse_program() {
            Ok(program) => {
                let mut compiler =
                    Factory::new(self.backend.unwrap(), program, self.filename.as_str());
                println!("Compiling to bytecode");
                compiler.bytecode_to_jit(&self.target);
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

    fn eval_interpretted(&self) -> Option<Result<Rc<Box<dyn Object>>, EvaluatorError>> {
        let source = self.source.clone();
        let environment = self.environment.clone();

        let mut parser = Parser::new(&source);
        let mut evaluated_result: Option<Result<Rc<Box<dyn Object>>, EvaluatorError>> = None;
        match parser.parse_program() {
            Ok(program) => {
                for stmt in program.stmts {
                    match evaluate_statement(&stmt, environment.clone()) {
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

    pub fn eval(&self) -> Option<Result<Rc<Box<dyn Object>>, EvaluatorError>> {
        if self.jit {
            return self.eval_jit();
        } else {
            return self.eval_interpretted();
        }
    }
}

pub fn evaluate_expression(
    expression: &Box<dyn Expression>,
    environment: Rc<RefCell<Environment>>,
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
        let left = evaluate_expression(&binary.left, environment.clone());
        let right: Result<Rc<Box<dyn Object>>, EvaluatorError> =
            evaluate_expression(&binary.right, environment.clone());
        return evaluate_binary_expression(binary.operator.clone(), left.unwrap(), right.unwrap());
    } else if let Some(if_expression) = value_any.downcast_ref::<IfExpression>() {
        return evaluate_condition_expression(if_expression, environment);
    } else if let Some(ident) = value_any.downcast_ref::<Identifier>() {
        return evaluate_identifier(ident, environment);
    } else if let Some(function) = value_any.downcast_ref::<FunctionLiteral>() {
        let parameters = function.parameters.clone();
        let body = function.body.clone();
        return Ok(Rc::new(Box::new(Function {
            parameters: parameters,
            body: body,
            env: Environment::new(),
        })));
    } else if let Some(call_expression) = value_any.downcast_ref::<CallExpression>() {
        let function = evaluate_expression(&call_expression.funtion, environment.clone())?;
        let args = eval_arg_expression(
            call_expression.parameters.clone(),
            function.clone(),
            environment.clone(),
        )?;
        return apply_function(function, args, environment.clone());
    } else if let Some(_null) = value_any.downcast_ref::<NullLiteral>() {
        return Ok(Rc::new(Box::new(NULL)));
    } else {
        return Err(EvaluatorError::new(
            String::from("Expression not found for eval"),
            None,
            None,
        ));
    }
}

pub fn evaluate_statement(
    statement: &Box<dyn Statement>,
    environment: Rc<RefCell<Environment>>,
) -> Result<Rc<Box<dyn Object>>, EvaluatorError> {
    let value_any = statement.as_any();
    if let Some(expr) = value_any.downcast_ref::<ExpressionStatement>() {
        return evaluate_expression(&expr.value, environment.clone());
    } else if let Some(block_statement) = value_any.downcast_ref::<BlockStatement>() {
        return evaluate_block_statement_ref(block_statement, environment.clone());
    } else if let Some(return_statement) = value_any.downcast_ref::<ReturnStatement>() {
        return evaluate_return_statement(return_statement, environment.clone());
    } else if let Some(let_statement) = value_any.downcast_ref::<LetStatement>() {
        return evaluate_let_statement(let_statement, environment.clone());
    } else {
        return Err(EvaluatorError::new(
            String::from("Requested Statement type not found"),
            None,
            None,
        ));
    }
}
