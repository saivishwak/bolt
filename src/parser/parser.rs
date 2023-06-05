#![allow(dead_code)]
use std::collections::HashMap;
use std::rc::Rc;

use super::ast;
use super::ast::Expression;
use super::types::{ParseError, ParseErrorKind, PrecedenceValue, Precedences};
use crate::lexer::lexer;
use crate::lexer::token::TokenType;
use crate::lexer::token::{self, Token};

pub struct Parser<'a> {
    pub lexer: lexer::Lexer<'a>,
    curr_token: Option<token::Token>,
    peek_token: Option<token::Token>,
    precedences: HashMap<token::TokenType, PrecedenceValue>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut p = Self {
            lexer: lexer::Lexer::new(source),
            curr_token: None,
            peek_token: None,
            precedences: HashMap::from([
                (TokenType::ASSIGN, Precedences::EQUALS as PrecedenceValue),
                (TokenType::NOTEQ, Precedences::EQUALS as PrecedenceValue),
                (TokenType::LT, Precedences::LESSGREATER as PrecedenceValue),
                (TokenType::GT, Precedences::LESSGREATER as PrecedenceValue),
                (TokenType::PLUS, Precedences::SUM as PrecedenceValue),
                (TokenType::MINUS, Precedences::SUM as PrecedenceValue),
                (TokenType::SLASH, Precedences::PRODUCT as PrecedenceValue),
                (TokenType::ASTERISK, Precedences::PRODUCT as PrecedenceValue),
                (TokenType::LPAREN, Precedences::CALL as PrecedenceValue),
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

    fn get_precedence_value(&self, precedence: &str) -> usize {
        let precedence = match precedence {
            "LOWEST" => Precedences::LOWEST,
            "CALL" => Precedences::CALL,
            "EQUALS" => Precedences::EQUALS,
            "LESSGREATER" => Precedences::LESSGREATER,
            "PREFIX" => Precedences::PREFIX,
            "PRODUCT" => Precedences::PRODUCT,
            "SUM" => Precedences::SUM,
            _ => Precedences::LOWEST,
        };
        return precedence as usize;
    }

    fn parse_statement(&mut self) -> Result<Box<dyn ast::Statement>, ParseError> {
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

    fn current_token(&self) -> Result<Token, ParseError> {
        let token = match self.curr_token.as_ref() {
            Some(tok) => Ok(tok.clone()),
            None => Err(ParseError {
                message: String::from("Error getting current token"),
                kind: ParseErrorKind::INTERNAL,
            }),
        };
        token
    }

    fn expect_peek_token(&mut self) -> Option<TokenType> {
        let mut flag = false;
        let peek_token = match self.peek_token.as_ref() {
            Some(token) => {
                flag = true;
                Some(token.token_type)
            }
            None => None,
        };
        if flag {
            self.next_token();
        }
        peek_token
    }

    fn parse_let_statement(&mut self) -> Result<Box<dyn ast::Statement>, ParseError> {
        let token = self.current_token().unwrap();

        if self.expect_peek_token().unwrap() != token::TokenType::IDENTIFIER {
            return Err(ParseError {
                message: String::from(format!(
                    "Invalid next token at line {}, expected to have IDENTIFIER",
                    token.line
                )),
                kind: ParseErrorKind::GENERIC,
            });
        }
        let identifier = ast::Identifier {
            token: self.curr_token.as_ref().unwrap().clone(),
            value: self.curr_token.as_ref().unwrap().literal.clone(),
        };

        if self.expect_peek_token().unwrap() != token::TokenType::ASSIGN {
            return Err(ParseError {
                message: String::from("Invalid next toekn, expected to have = Operator"),
                kind: ParseErrorKind::GENERIC,
            });
        }
        self.next_token();
        let expression = self.parse_expression(self.get_precedence_value("LOWEST"));
        let stmt = ast::LetStatement {
            token: token,
            identifier: identifier,
            value: expression.unwrap(),
        };
        Ok(Box::new(stmt))
    }

    fn parse_expression(
        &mut self,
        precedence: usize,
    ) -> Result<Box<dyn ast::Expression>, ParseError> {
        let curr_token = self.current_token().unwrap();
        let mut left_expr: Box<dyn Expression> = match curr_token.token_type {
            //All prefix parsers
            TokenType::INT => Box::new(ast::IntegerLiteral {
                token: curr_token.clone(),
                value: curr_token.literal.parse::<f32>().unwrap(),
            }),
            TokenType::IDENTIFIER => self.parse_identifier_expression(),
            TokenType::BANG | TokenType::MINUS => self.parse_prefix_expression(),
            TokenType::LPAREN => match self.parse_group_expression() {
                Ok(token) => token,
                Err(e) => {
                    return Err(e);
                }
            },
            _ => {
                return Err(ParseError {
                    message: String::from("No Method for parsing token"),
                    kind: ParseErrorKind::GENERIC,
                })
            }
        };
        loop {
            if self.expect_peek_token().unwrap() != token::TokenType::SEMICOLON
                && precedence < self.peek_precedence()
            {
                let peek_token = self.peek_token.as_ref().unwrap().clone();
                match peek_token.token_type {
                    TokenType::PLUS
                    | TokenType::MINUS
                    | TokenType::SLASH
                    | TokenType::ASTERISK
                    | TokenType::EQ
                    | TokenType::NOTEQ
                    | TokenType::GT
                    | TokenType::LT => {
                        left_expr = self.parse_infix_expression(Rc::new(left_expr));
                    }
                    _ => {
                        break;
                    }
                }
            } else {
                break;
            }
        }

        Ok(left_expr)
    }

    fn parse_group_expression(&mut self) -> Result<Box<dyn Expression>, ParseError> {
        self.next_token();
        let exp = self.parse_expression(self.get_precedence_value("LOWEST"));
        if self.expect_peek_token().unwrap() != TokenType::RPAREN {
            return Err(ParseError {
                message: String::from("Error parsing group"),
                kind: ParseErrorKind::GENERIC,
            });
        }
        return exp;
    }

    fn parse_infix_expression(&mut self, left: Rc<Box<dyn Expression>>) -> Box<dyn Expression> {
        let curr_token = self.current_token().unwrap();
        let literal = curr_token.literal.clone();
        let precedence = self.current_precedence();
        self.next_token();
        let right = self.parse_expression(precedence).unwrap();
        let expression = ast::BinaryExpression {
            token: curr_token,
            operator: literal,
            left: left,
            right: right,
        };
        return Box::new(expression);
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

        let right = self.parse_expression(self.get_precedence_value("PREFIX"));
        Box::new(ast::PrefixExpression {
            token: curr_token,
            operator: operator,
            right: right.unwrap(),
        })
    }

    fn current_precedence(&self) -> PrecedenceValue {
        let curr_token = self.curr_token.as_ref().unwrap().clone();
        let p: PrecedenceValue;
        match self.precedences.get(&curr_token.token_type) {
            Some(precedence) => {
                p = *precedence;
            }
            None => {
                p = self.get_precedence_value("LOWEST");
            }
        }
        return p;
    }

    fn peek_precedence(&self) -> PrecedenceValue {
        let peek_token = self.peek_token.as_ref().unwrap().clone();
        let p: PrecedenceValue;
        match self.precedences.get(&peek_token.token_type) {
            Some(precedence) => {
                p = *precedence;
            }
            None => {
                p = self.get_precedence_value("LOWEST");
            }
        }
        return p;
    }

    fn parse_expression_statement(&mut self) -> Result<Box<dyn ast::Statement>, ParseError> {
        let curr_token = self.curr_token.as_ref().unwrap().clone();
        let expr = self
            .parse_expression(self.get_precedence_value("LOWEST"))
            .unwrap();

        if self.peek_token.as_ref().unwrap().token_type == token::TokenType::SEMICOLON {
            self.next_token();
        }

        Ok(Box::new(ast::ExpressionStatement {
            token: curr_token,
            value: expr,
        }))
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, ParseError> {
        let mut program = ast::Program { stmts: vec![] };
        let stmt = self.parse_statement();
        program.stmts.push(stmt.unwrap());
        Ok(program)
    }
}
