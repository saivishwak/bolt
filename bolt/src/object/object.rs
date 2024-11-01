use crate::{
    evaluator::environment::Environment,
    parser::ast::{BlockStatement, Identifier},
};

use super::types::ObjectType;
use core::fmt::Debug;
use std::{any::Any, cell::RefCell, rc::Rc};

pub trait Object
where
    Self: Debug,
{
    fn get_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub struct Interger {
    pub value: f64,
}

impl Object for Interger {
    fn get_type(&self) -> ObjectType {
        ObjectType::INTERGER
    }
    fn inspect(&self) -> String {
        self.value.to_string()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct BooleanObj {
    pub value: bool,
}

impl Object for BooleanObj {
    fn get_type(&self) -> ObjectType {
        ObjectType::BOOLEAN
    }
    fn inspect(&self) -> String {
        self.value.to_string()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct Return {
    pub value: Rc<Box<dyn Object>>,
}

impl Object for Return {
    fn get_type(&self) -> ObjectType {
        ObjectType::RETURN
    }
    fn inspect(&self) -> String {
        return self.value.inspect();
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct Function {
    pub parameters: Rc<Vec<Identifier>>,
    pub body: Rc<Box<BlockStatement>>,
    pub env: Rc<RefCell<Environment>>,
}

impl Object for Function {
    fn get_type(&self) -> ObjectType {
        ObjectType::FUNCTION
    }

    #[allow(unused_assignments)]
    fn inspect(&self) -> String {
        let mut result = String::new();
        let mut params = vec![];
        for param in self.parameters.as_ref() {
            params.push(param.value.clone());
        }
        result = format!("fn({})", params.join(","));
        return result;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct Null {}

impl Object for Null {
    fn get_type(&self) -> ObjectType {
        ObjectType::NULL
    }
    fn inspect(&self) -> String {
        String::from("null")
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
