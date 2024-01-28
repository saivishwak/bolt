#![allow(dead_code)]
use std::collections::HashMap;
use std::rc::Rc;

use super::ast::{self, BlockStatement};
use super::ast::{Expression, Statement};
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
                (TokenType::EQ, Precedences::EQUALS as PrecedenceValue),
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
            token::TokenType::LET => return self.parse_let_statement(),
            token::TokenType::RETURN => self.parse_return_statement(),
            _ => return self.parse_expression_statement(),
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

    fn peek_token(&self) -> Option<TokenType> {
        let peek_token = match self.peek_token.as_ref() {
            Some(token) => Some(token.token_type),
            None => None,
        };
        peek_token
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

    fn parse_return_statement(&mut self) -> Result<Box<dyn ast::Statement>, ParseError> {
        let current_token = self.current_token().unwrap();
        //Skip return token
        self.next_token();
        let expr = self
            .parse_expression(self.get_precedence_value("LOWEST"))
            .unwrap();

        if self.peek_token().unwrap() == TokenType::SEMICOLON {
            self.next_token();
        }

        Ok(Box::new(ast::ReturnStatement {
            token: current_token.clone(),
            value: expr,
        }))
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

    fn parse_expression_statement(&mut self) -> Result<Box<dyn ast::Statement>, ParseError> {
        let curr_token = self.current_token().unwrap();
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
                Ok(expr) => expr,
                Err(e) => {
                    return Err(e);
                }
            },
            TokenType::TRUE | TokenType::FALSE => self.parse_boolean_expression(),
            TokenType::IF => match self.parse_if_expression() {
                Ok(expr) => expr,
                Err(e) => {
                    return Err(e);
                }
            },
            TokenType::FUNCTION => match self.parse_function_literal() {
                Ok(expr) => expr,
                Err(e) => {
                    return Err(e);
                }
            },
            _ => {
                return Err(ParseError {
                    message: String::from(format!(
                        "No Method for parsing prefix token {:?}",
                        curr_token.token_type
                    )),
                    kind: ParseErrorKind::GENERIC,
                })
            }
        };
        loop {
            let peek_precedence = self.peek_precedence();
            if self.peek_token().unwrap() != token::TokenType::SEMICOLON
                && precedence < peek_precedence
            {
                let peek_token = self.peek_token.as_ref().unwrap().clone();
                //Move the token to next that is the prefix operator
                self.next_token();
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
                    TokenType::LPAREN => {
                        left_expr = self.parse_call_expression(Rc::new(left_expr)).unwrap();
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

    fn parse_function_literal(&mut self) -> Result<Box<dyn Expression>, ParseError> {
        let curren_token = self.current_token().unwrap();
        if self.expect_peek_token().unwrap() != TokenType::LPAREN {
            return Err(ParseError {
                message: String::from("Expected ("),
                kind: ParseErrorKind::GENERIC,
            });
        }
        self.next_token();

        //parse parameters
        let mut parameters: Vec<ast::Identifier> = vec![];
        loop {
            let curr_token = self.current_token().unwrap();
            if curr_token.token_type == TokenType::RPAREN {
                break;
            }
            // We are not caring if the function params start with , we skip it
            if curr_token.token_type == TokenType::COMMA {
                self.next_token();
                continue;
            }
            let ident = ast::Identifier {
                token: curr_token.clone(),
                value: curr_token.literal,
            };
            parameters.push(ident);
            self.next_token();
        }

        if self.expect_peek_token().unwrap() != TokenType::LBRACE {
            return Err(ParseError {
                message: String::from("Expected LBRACE"),
                kind: ParseErrorKind::GENERIC,
            });
        }
        self.next_token();

        let body = self.parse_block_statement().unwrap();

        Ok(Box::new(ast::FunctionLiteral {
            token: curren_token,
            parameters: parameters,
            body: body,
        }))
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

    fn parse_call_expression(
        &mut self,
        left: Rc<Box<dyn Expression>>,
    ) -> Result<Box<dyn Expression>, ParseError> {
        let curr_token = self.current_token().unwrap();
        self.next_token();

        let mut parameters: Vec<Box<dyn Expression>> = vec![];
        loop {
            let curr_token = self.current_token().unwrap();
            if curr_token.token_type == TokenType::RPAREN {
                break;
            }
            // We are not caring if the function params start with , we skip it
            if curr_token.token_type == TokenType::COMMA {
                self.next_token();
                continue;
            }
            if curr_token.token_type == TokenType::LPAREN {
                self.next_token();
            }
            let ident = self
                .parse_expression(self.get_precedence_value("LOWEST"))
                .unwrap();
            self.next_token();
            parameters.push(ident);
        }

        let call_expression = ast::CallExpression {
            token: curr_token,
            funtion: left.clone(),
            parameters: parameters,
        };
        return Ok(Box::new(call_expression));
    }

    fn parse_if_expression(&mut self) -> Result<Box<ast::IfExpression>, ParseError> {
        let current_token = self.current_token().unwrap();
        //Skip if token
        if self.expect_peek_token().unwrap() != TokenType::LPAREN {
            return Err(ParseError {
                message: String::from(format!("Expected to have ( at line {}", current_token.line)),
                kind: ParseErrorKind::GENERIC,
            });
        };
        self.next_token();

        let condition = self
            .parse_expression(self.get_precedence_value("LOWEST"))
            .unwrap();

        self.next_token();

        if self.current_token().unwrap().token_type != TokenType::RPAREN {
            return Err(ParseError {
                message: String::from(format!("Expected to have ) at line {}", current_token.line)),
                kind: ParseErrorKind::GENERIC,
            });
        };

        let peek_token = self.expect_peek_token().unwrap();
        if peek_token != TokenType::LBRACE {
            return Err(ParseError {
                message: String::from(format!(
                    "Expected to have L Brace at line {} but found {:?}",
                    current_token.line, peek_token
                )),
                kind: ParseErrorKind::GENERIC,
            });
        };
        self.next_token();

        let consequence = self.parse_block_statement().unwrap();

        //skip } token
        self.next_token();

        let mut alternate = None;
        //Parse the else condition as well
        if self.current_token().unwrap().token_type == TokenType::ELSE {
            if self.expect_peek_token().unwrap() != TokenType::LBRACE {
                return Err(ParseError {
                    message: String::from(format!(
                        "Expected to have LBrace at line {}",
                        current_token.line
                    )),
                    kind: ParseErrorKind::GENERIC,
                });
            }
            //Skip { token
            self.next_token();
            alternate = Some(self.parse_block_statement().unwrap());
        };

        Ok(Box::new(ast::IfExpression {
            token: current_token,
            condition: condition,
            consequence: consequence,
            alternate: alternate,
        }))
    }

    fn parse_identifier_expression(&mut self) -> Box<dyn ast::Expression> {
        let curr_token = self.current_token().unwrap();
        let literal = curr_token.literal.clone();
        Box::new(ast::Identifier {
            token: curr_token,
            value: literal,
        })
    }

    fn parse_boolean_expression(&mut self) -> Box<dyn ast::Expression> {
        let current_token = self.current_token().unwrap();
        let current_token_type = current_token.token_type;
        Box::new(ast::Boolean {
            token: current_token,
            value: current_token_type == TokenType::TRUE,
        })
    }

    fn parse_prefix_expression(&mut self) -> Box<dyn ast::Expression> {
        let curr_token = self.current_token().unwrap();
        let operator = curr_token.literal.clone();

        self.next_token();

        let right = self.parse_expression(self.get_precedence_value("PREFIX"));
        Box::new(ast::PrefixExpression {
            token: curr_token,
            operator: operator,
            right: right.unwrap(),
        })
    }

    fn parse_block_statement(&mut self) -> Result<Box<BlockStatement>, ParseError> {
        let current_token = self.current_token().unwrap();
        let mut stmts: Vec<Box<dyn Statement>> = vec![];
        loop {
            let current_token_type = self.current_token().unwrap().token_type;
            if current_token_type == TokenType::RBRACE || current_token_type == TokenType::EOF {
                break;
            }
            let stmt = self.parse_statement().unwrap();
            stmts.push(stmt);
            self.next_token();
            if self.current_token().unwrap().token_type == TokenType::SEMICOLON {
                self.next_token();
            } else {
                //ToDo - Better Error handling and also is semicolon madatory in bolt?
                // return Err(ParseError {
                //     message: String::from("Semicolon missing for statement"),
                //     kind: ParseErrorKind::GENERIC,
                // });
            }
            println!("{:?}", self.current_token().unwrap());
        }
        Ok(Box::new(BlockStatement {
            token: current_token,
            statements: stmts,
        }))
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, ParseError> {
        let mut program = ast::Program { stmts: vec![] };
        let stmt = self.parse_statement();
        program.stmts.push(stmt.unwrap());
        Ok(program)
    }
}
