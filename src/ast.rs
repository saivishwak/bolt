#![allow(dead_code)]
use crate::token;
use core::fmt::Debug;
use std::rc::Rc;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node
where
    Self: Debug,
{
    fn print(&self) -> String {
        let result = format!("{:?}", self);
        result
    }
}

pub trait Expression: Node
where
    Self: Debug,
{
    fn print(&self) -> String {
        let result = format!("{:?}", self);
        result
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: token::Token,
    pub identifier: Identifier,
    pub value: Box<dyn Expression>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
}

impl Statement for LetStatement {}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: token::Token,
    pub value: Box<dyn Expression>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
}

impl Statement for ExpressionStatement {}

#[derive(Debug)]
pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
}

impl Expression for Identifier {}

//Expression Nodes
#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: token::Token,
    pub value: f32,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
}

impl Expression for IntegerLiteral {}

#[derive(Debug)]
pub struct PrefixExpression {
    pub token: token::Token,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
}

impl Expression for PrefixExpression {}

//Expression Nodes
#[derive(Debug)]
pub struct BinaryExpression {
    pub token: token::Token,
    pub operator: String,
    pub left: Rc<Box<dyn Expression>>,
    pub right: Box<dyn Expression>,
}

impl Node for BinaryExpression {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
}

impl Expression for BinaryExpression {}

#[derive(Debug)]
pub struct Program {
    pub stmts: Vec<Box<dyn Statement>>,
}

impl Program {}
