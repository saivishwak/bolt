#![allow(dead_code)]
use std::collections::HashMap;

use super::ast;
use super::lexer;
use crate::ast::Expression;
use crate::token;
use crate::token::TokenType;

const LOWEST: i32 = 0;
const EQUALS: i32 = 1;
const LESSGREATER: i32 = 2;
const SUM: i32 = 3;
const PRODUCT: i32 = 4;
const PREFIX: i32 = 5;
const CALL: i32 = 6;

pub struct Parser<'a> {
    pub lexer: lexer::Lexer<'a>,
    curr_token: Option<token::Token>,
    peek_token: Option<token::Token>,
    precedences: HashMap<token::TokenType, i32>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut p = Self {
            lexer: lexer::Lexer::new(source),
            curr_token: None,
            peek_token: None,
            precedences: HashMap::from([
                (TokenType::ASSIGN, EQUALS),
                (TokenType::NOTEQ, EQUALS),
                (TokenType::LT, LESSGREATER),
                (TokenType::GT, LESSGREATER),
                (TokenType::PLUS, SUM),
                (TokenType::MINUS, SUM),
                (TokenType::SLASH, PRODUCT),
                (TokenType::ASTERISK, PRODUCT),
                (TokenType::LPAREN, CALL),
            ]),
        };
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        let pt: Option<token::Token>;
        match &self.peek_token {
            Some(token) => pt = Some(token.clone()),
            None => pt = None,
        }
        self.curr_token = pt;
        self.peek_token = Some(self.lexer.next_token());
    }

    fn parse_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        let curr_token = self.curr_token.as_ref().unwrap();
        match curr_token.token_type {
            token::TokenType::LET => {
                return self.parse_let_statement();
            }
            _ => {
                return self.parse_expression_statement();
            }
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        let token = self.curr_token.as_ref().unwrap().clone();

        if self.peek_token.as_ref().unwrap().token_type != token::TokenType::IDENTIFIER {
            return None;
        }
        self.next_token();
        let identifier = ast::Identifier {
            token: self.curr_token.as_ref().unwrap().clone(),
            value: self.curr_token.as_ref().unwrap().literal.clone(),
        };

        if self.peek_token.as_ref().unwrap().token_type != token::TokenType::ASSIGN {
            return None;
        }
        self.next_token();
        self.next_token();
        let expression = self.parse_expression(LOWEST);

        let stmt = ast::LetStatement {
            token: token,
            identifier: identifier,
            value: expression.unwrap(),
        };
        Some(Box::new(stmt))
    }

    fn parse_expression(&mut self, precedence: i32) -> Option<Box<dyn ast::Expression>> {
        let curr_token = self.curr_token.as_ref().unwrap();
        let left_expr: Box<dyn Expression> = match curr_token.token_type {
            //All prefix parsers
            token::TokenType::INT => Box::new(ast::IntegerLiteral {
                token: curr_token.clone(),
                value: curr_token.literal.parse::<f32>().unwrap(),
            }),
            token::TokenType::IDENTIFIER => self.parse_identifier_expression(),
            token::TokenType::BANG => self.parse_prefix_expression(),
            token::TokenType::MINUS => self.parse_prefix_expression(),
            _ => return None,
        };

        /*loop {
            if self.peek_token.as_ref().unwrap().token_type != token::TokenType::SEMICOLON
                && precedence < self.peek_precedence()
            {
                //
            }
        }*/

        Some(left_expr)
    }

    fn parse_identifier_expression(&mut self) -> Box<dyn ast::Expression> {
        let curr_token = self.curr_token.as_ref().unwrap().clone();
        let literal = curr_token.literal.clone();
        Box::new(ast::Identifier {
            token: curr_token,
            value: literal,
        })
    }

    fn parse_prefix_expression(&mut self) -> Box<dyn ast::Expression> {
        let curr_token = self.curr_token.as_ref().unwrap().clone();
        let operator = curr_token.literal.clone();

        self.next_token();

        let right = self.parse_expression(PREFIX);
        Box::new(ast::PrefixExpression {
            token: curr_token,
            operator: operator,
            right: right.unwrap(),
        })
    }

    fn current_precedence(&self) -> i32 {
        let curr_token = self.curr_token.as_ref().unwrap().clone();
        let p: i32;
        match self.precedences.get(&curr_token.token_type) {
            Some(precedence) => {
                p = *precedence;
            }
            None => {
                p = LOWEST;
            }
        }
        return p;
    }

    fn peek_precedence(&self) -> i32 {
        let peek_token = self.peek_token.as_ref().unwrap().clone();
        let p: i32;
        match self.precedences.get(&peek_token.token_type) {
            Some(precedence) => {
                p = *precedence;
            }
            None => {
                p = LOWEST;
            }
        }
        return p;
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        let curr_token = self.curr_token.as_ref().unwrap().clone();
        let expr = self.parse_expression(LOWEST).unwrap();

        if self.peek_token.as_ref().unwrap().token_type == token::TokenType::SEMICOLON {
            self.next_token();
        }

        Some(Box::new(ast::ExpressionStatement {
            token: curr_token,
            value: expr,
        }))
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program { stmts: vec![] };
        let stmt = self.parse_statement();
        program.stmts.push(stmt.unwrap());

        program
    }
}
