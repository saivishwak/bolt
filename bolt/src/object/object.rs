use super::types::ObjectType;
use core::fmt::Debug;
use std::{any::Any, rc::Rc};

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
    pub value: f32,
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
