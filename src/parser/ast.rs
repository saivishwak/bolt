#![allow(dead_code)]
use crate::lexer::token;
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
pub struct ReturnStatement {
    pub token: token::Token,
    pub value: Box<dyn Expression>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
}

impl Statement for ReturnStatement {}

#[derive(Debug)]
pub struct BlockStatement {
    pub token: token::Token,
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
}

impl Statement for BlockStatement {}

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

//Expression Nodes
#[derive(Debug)]
pub struct Boolean {
    pub token: token::Token,
    pub value: bool,
}

impl Node for Boolean {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
}

impl Expression for Boolean {}

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

#[derive(Debug)]
pub struct IfExpression {
    pub token: token::Token,
    pub condition: Box<dyn Expression>,
    pub consequence: Box<BlockStatement>,
    pub alternate: Option<Box<BlockStatement>>,
}

impl Node for IfExpression {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
}

impl Expression for IfExpression {}

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
