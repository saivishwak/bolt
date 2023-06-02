#![allow(dead_code)]
use super::ast;
use super::lexer;
use crate::token;

pub struct Parser<'a> {
    pub lexer: lexer::Lexer<'a>,
    curr_token: Option<token::Token>,
    peek_token: Option<token::Token>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut p = Self {
            lexer: lexer::Lexer::new(source),
            curr_token: None,
            peek_token: None,
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
        let expression = self.parse_expression();

        let stmt = ast::LetStatement {
            token: token,
            identifier: identifier,
            value: expression,
        };
        Some(Box::new(stmt))
    }

    fn parse_expression(&mut self) -> Box<ast::Expression> {
        Box::new(ast::Expression {})
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        let expr = self.parse_expression();

        if self.peek_token.as_ref().unwrap().token_type == token::TokenType::SEMICOLON {
            self.next_token();
        }

        Some(expr)
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program { stmts: vec![] };
        let stmt = self.parse_statement();
        program.stmts.push(stmt.unwrap());

        program
    }
}
