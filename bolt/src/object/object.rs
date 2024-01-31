use super::types::ObjectType;
use core::fmt::Debug;
use std::any::Any;

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
    pub v: f32,
}

impl Object for Interger {
    fn get_type(&self) -> ObjectType {
        ObjectType::INTERGER
    }
    fn inspect(&self) -> String {
        self.v.to_string()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct BooleanObj {
    pub v: bool,
}

impl Object for BooleanObj {
    fn get_type(&self) -> ObjectType {
        ObjectType::BOOLEAN
    }
    fn inspect(&self) -> String {
        self.v.to_string()
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
