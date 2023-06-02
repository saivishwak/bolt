#![allow(dead_code)]
use crate::token;
use std::fmt;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn print(&self) -> String;
}

impl fmt::Debug for dyn Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement your desired debug output here
        write!(f, "{}", self.print())
    }
}

pub trait Expression: Node {
    fn print(&self) -> String;
}

impl fmt::Debug for dyn Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement your desired debug output here
        write!(f, "{}", self.print())
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

impl Statement for LetStatement {
    fn print(&self) -> String {
        let result = format!("{:?}", self);
        result
    }
}

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

impl Statement for Identifier {
    fn print(&self) -> String {
        let result = format!("{:?}", self);
        result
    }
}

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

impl Expression for IntegerLiteral {
    fn print(&self) -> String {
        let result = format!("{:?}", self);
        result
    }
}

#[derive(Debug)]
pub struct Program {
    pub stmts: Vec<Box<dyn Statement>>,
}

impl Program {
    /*
    pub fn add_token(&mut self) {
        self.stmts.push(Box::new(LetStatement {
            token: token::Token {
                token_type: token::TokenType::LET,
                literal: String::from(""),
                line: 12,
            },
            value: Expression {},
        }))
    }
    */
}
