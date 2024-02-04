use std::collections::HashMap;
use std::rc::Rc;

use super::ast::{self, BlockStatement};
use super::ast::{Expression, Statement};
use super::types::{PrecedenceValue, Precedences};
use crate::error::{BoltError, BoltErrorType, ParseError};
use crate::lexer::lexer;
use crate::lexer::token::TokenType;
use crate::lexer::token::{self, Token};

// use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
pub struct Parser<'a> {
    pub lexer: lexer::Lexer<'a>,
    curr_token: Option<token::Token>,
    peek_token: Option<token::Token>,
    precedences: HashMap<token::TokenType, PrecedenceValue>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut parser = Self {
            lexer: lexer::Lexer::new(source),
            curr_token: None,
            peek_token: None,
            precedences: HashMap::from([
                (TokenType::ASSIGN, Precedences::EQUALS as PrecedenceValue),
                (TokenType::NOTEQ, Precedences::EQUALS as PrecedenceValue),
                (TokenType::EQ, Precedences::EQUALS as PrecedenceValue),
                (TokenType::LT, Precedences::LESSGREATER as PrecedenceValue),
                (TokenType::GT, Precedences::LESSGREATER as PrecedenceValue),
                (TokenType::GTEQ, Precedences::LESSGREATER as PrecedenceValue),
                (TokenType::LTEQ, Precedences::LESSGREATER as PrecedenceValue),
                (TokenType::PLUS, Precedences::SUM as PrecedenceValue),
                (TokenType::MINUS, Precedences::SUM as PrecedenceValue),
                (TokenType::SLASH, Precedences::PRODUCT as PrecedenceValue),
                (TokenType::ASTERISK, Precedences::PRODUCT as PrecedenceValue),
                (TokenType::LPAREN, Precedences::CALL as PrecedenceValue),
            ]),
        };
        //Move 2 steps to make the first token as current token
        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        let peek_token: Option<token::Token>;
        match &self.peek_token {
            Some(token) => peek_token = Some(token.clone()),
            None => peek_token = None,
        }
        self.curr_token = peek_token;
        self.peek_token = Some(self.lexer.next_token());
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, ParseError> {
        let mut program = ast::Program { stmts: vec![] };
        loop {
            let statement: Result<Box<dyn Statement>, ParseError> = self.parse_statement();

            match statement {
                Ok(value) => {
                    program.stmts.push(value);
                }
                Err(e) => match e.get_type() {
                    BoltErrorType::EOF => {
                        //Break the loop for EOF
                        break;
                    }
                    _ => {
                        return Err(e);
                    }
                },
            }
        }
        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Box<dyn ast::Statement>, ParseError> {
        let curr_token = self.get_current_token()?;
        match curr_token.token_type {
            token::TokenType::LET => return self.parse_let_statement(),
            token::TokenType::RETURN => return self.parse_return_statement(),
            _ => return self.parse_expression_statement(),
        }
    }

    fn parse_return_statement(&mut self) -> Result<Box<dyn ast::Statement>, ParseError> {
        //Get the Return Token and skip it
        let current_token = self.get_current_token_and_skip()?;
        let expr = self.parse_expression(self.get_precedence_value("LOWEST"))?;

        self.skip_peek_semicolon_token();

        Ok(Box::new(ast::ReturnStatement {
            token: current_token.clone(),
            value: expr,
        }))
    }

    fn parse_let_statement(&mut self) -> Result<Box<dyn ast::Statement>, ParseError> {
        let token = self.get_current_token()?;
        if !self.expect_peek_token_with_type(token::TokenType::IDENTIFIER) {
            return Err(ParseError::new(
                String::from(format!(
                    "Invalid next token at line {}, expected to have IDENTIFIER",
                    token.line,
                )),
                None,
                Some(token.line),
            ));
        };

        let identifier_token = self.get_current_token()?;
        let identifier = ast::Identifier {
            token: identifier_token.clone(),
            value: identifier_token.literal.clone(),
        };
        if !self.expect_peek_token_with_type(token::TokenType::ASSIGN) {
            return Err(ParseError::new(
                String::from("Invalid next toekn, expected to have = Operator"),
                None,
                None,
            ));
        }
        //Skip the assign Token
        self.next_token();

        let expression = self.parse_expression(self.get_precedence_value("LOWEST"))?;
        let stmt = ast::LetStatement {
            token: token,
            identifier: identifier,
            value: expression,
        };
        self.skip_current_semicolon_token();

        Ok(Box::new(stmt))
    }

    fn parse_expression_statement(&mut self) -> Result<Box<dyn ast::Statement>, ParseError> {
        let current_token = self.get_current_token()?;
        match self.parse_expression(self.get_precedence_value("LOWEST")) {
            Ok(expr) => {
                self.next_token();
                return Ok(Box::new(ast::ExpressionStatement {
                    token: current_token,
                    value: expr,
                }));
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    fn parse_expression(
        &mut self,
        precedence: usize,
    ) -> Result<Box<dyn ast::Expression>, ParseError> {
        let current_token = self.get_current_token()?;

        //Parse all prefix expresssions
        let mut left_expr: Box<dyn Expression> = match current_token.token_type {
            TokenType::INT => {
                if let Ok(float_value) = current_token.literal.parse::<f32>() {
                    Box::new(ast::IntegerLiteral {
                        token: current_token.clone(),
                        value: float_value,
                    })
                } else {
                    return Err(ParseError::new(
                        String::from("Error converting number to float"),
                        None,
                        Some(current_token.line),
                    ));
                }
            }
            TokenType::NULL => Box::new(ast::NullLiteral {}),
            TokenType::IDENTIFIER => match self.parse_identifier_expression() {
                Ok(identifier_expression) => identifier_expression,
                Err(e) => {
                    return Err(e);
                }
            },
            TokenType::BANG | TokenType::MINUS => match self.parse_prefix_expression() {
                Ok(prefix_expression) => prefix_expression,
                Err(e) => return Err(e),
            },
            TokenType::LPAREN => match self.parse_group_expression() {
                Ok(expr) => expr,
                Err(e) => {
                    return Err(e);
                }
            },
            TokenType::TRUE | TokenType::FALSE => match self.parse_boolean_expression() {
                Ok(boolean_expression) => boolean_expression,
                Err(e) => return Err(e),
            },
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
            TokenType::EOF => {
                return Err(ParseError::new(
                    String::from("EOF"),
                    Some(BoltErrorType::EOF),
                    Some(current_token.line),
                ));
            }
            _ => {
                return Err(ParseError::new(
                    String::from(format!(
                        "No Method for parsing prefix token {:?}",
                        current_token.token_type
                    )),
                    None,
                    Some(current_token.line),
                ))
            }
        };
        loop {
            let peek_precedence = self.peek_precedence()?;
            if !self.expect_peek_token_with_type_and_no_advance(token::TokenType::SEMICOLON)
                && precedence < peek_precedence
            {
                if let Some(peek_token) = self.get_peek_token() {
                    //Move the token to next that is the prefix operator
                    self.next_token();
                    match peek_token.token_type {
                        TokenType::PLUS
                        | TokenType::MINUS
                        | TokenType::SLASH
                        | TokenType::ASTERISK
                        | TokenType::EQ
                        | TokenType::LTEQ
                        | TokenType::GTEQ
                        | TokenType::NOTEQ
                        | TokenType::GT
                        | TokenType::LT => {
                            left_expr = self.parse_infix_expression(Rc::new(left_expr))?;
                        }
                        TokenType::LPAREN => {
                            left_expr = self.parse_call_expression(Rc::new(left_expr))?
                        }
                        _ => {
                            break;
                        }
                    }
                } else {
                    return Err(ParseError::new(
                        String::from("Expected peek token but found None"),
                        None,
                        Some(current_token.line),
                    ));
                }
            } else {
                break;
            }
        }

        self.skip_current_semicolon_token();
        self.skip_peek_semicolon_token();

        Ok(left_expr)
    }

    fn parse_function_literal(&mut self) -> Result<Box<dyn Expression>, ParseError> {
        let curren_token = self.get_current_token()?;
        if !self.expect_peek_token_with_type(TokenType::LPAREN) {
            return Err(ParseError::new(String::from("Expected ("), None, None));
        }
        //Skip the LPAREN Token
        self.next_token();

        //parse parameters
        let mut parameters: Vec<ast::Identifier> = vec![];
        loop {
            let curr_token = self.get_current_token()?;
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

        if !self.expect_peek_token_with_type(TokenType::LBRACE) {
            return Err(ParseError::new(String::from("Expected LBRACE"), None, None));
        }
        // Skip the LBRACE Token
        self.next_token();

        let body = self.parse_block_statement()?;

        Ok(Box::new(ast::FunctionLiteral {
            token: curren_token,
            parameters: Rc::new(parameters),
            body: Rc::new(body),
        }))
    }

    fn parse_group_expression(&mut self) -> Result<Box<dyn Expression>, ParseError> {
        self.next_token();
        let exp = self.parse_expression(self.get_precedence_value("LOWEST"));
        if !self.expect_peek_token_with_type(TokenType::RPAREN) {
            return Err(ParseError::new(
                String::from("Error parsing group"),
                None,
                None,
            ));
        }
        return exp;
    }

    fn parse_infix_expression(
        &mut self,
        left: Rc<Box<dyn Expression>>,
    ) -> Result<Box<dyn Expression>, ParseError> {
        let current_token = self.get_current_token()?;
        let literal = current_token.literal.clone();
        let precedence = self.current_precedence()?;
        // Skip the operator token
        self.next_token();

        let right = self.parse_expression(precedence)?;
        let expression = ast::BinaryExpression {
            token: current_token,
            operator: literal,
            left: left,
            right: right,
        };
        return Ok(Box::new(expression));
    }

    fn parse_call_expression(
        &mut self,
        left: Rc<Box<dyn Expression>>,
    ) -> Result<Box<dyn Expression>, ParseError> {
        let curr_token = self.get_current_token()?;
        self.next_token();

        let mut parameters: Vec<Box<dyn Expression>> = vec![];
        loop {
            let curr_token = self.get_current_token()?;
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
            let ident = self.parse_expression(self.get_precedence_value("LOWEST"))?;
            self.next_token();
            parameters.push(ident);
        }

        let call_expression = ast::CallExpression {
            token: curr_token,
            funtion: left.clone(),
            parameters: Rc::new(parameters),
        };

        return Ok(Box::new(call_expression));
    }

    fn parse_if_expression(&mut self) -> Result<Box<ast::IfExpression>, ParseError> {
        let current_token = self.get_current_token()?;
        //Skip if token
        if !self.expect_peek_token_with_type(TokenType::LPAREN) {
            return Err(ParseError::new(
                String::from(format!("Expected to have ( at line {}", current_token.line)),
                None,
                None,
            ));
        };

        //Skip the LPAREN Token
        self.next_token();

        let condition = self.parse_expression(self.get_precedence_value("LOWEST"))?;

        self.next_token();

        if !self.check_current_token_match(TokenType::RPAREN) {
            return Err(ParseError::new(
                String::from(format!("Expected to have ) at line {}", current_token.line)),
                None,
                None,
            ));
        };

        if !self.expect_peek_token_with_type(TokenType::LBRACE) {
            return Err(ParseError::new(
                String::from(format!(
                    "Expected to have L Brace at line {} but found something else",
                    current_token.line
                )),
                None,
                None,
            ));
        };

        //Skip the LEFTBRACE Token
        self.next_token();
        let consequence = self.parse_block_statement()?;
        self.skip_current_semicolon_token();

        let mut alternate = None;
        //Parse the else condition as well
        if self.check_current_token_match(TokenType::ELSE) {
            if !self.expect_peek_token_with_type(TokenType::LBRACE) {
                return Err(ParseError::new(
                    String::from(format!(
                        "Expected to have LBrace at line {}",
                        current_token.line
                    )),
                    None,
                    None,
                ));
            }
            //Skip { token
            self.next_token();
            alternate = Some(self.parse_block_statement()?);
        };

        self.skip_current_semicolon_token();

        Ok(Box::new(ast::IfExpression {
            token: current_token,
            condition: condition,
            consequence: consequence,
            alternate: alternate,
        }))
    }

    fn parse_identifier_expression(&mut self) -> Result<Box<dyn ast::Expression>, ParseError> {
        let current_token = self.get_current_token()?;
        let literal = current_token.literal.clone();
        return Ok(Box::new(ast::Identifier {
            token: current_token,
            value: literal,
        }));
    }

    fn parse_boolean_expression(&mut self) -> Result<Box<dyn ast::Expression>, ParseError> {
        let current_token = self.get_current_token()?;
        let current_token_type = current_token.token_type;
        Ok(Box::new(ast::Boolean {
            token: current_token,
            value: current_token_type == TokenType::TRUE,
        }))
    }

    fn parse_prefix_expression(&mut self) -> Result<Box<dyn ast::Expression>, ParseError> {
        let curr_token = self.get_current_token()?;
        let operator = curr_token.literal.clone();

        self.next_token();

        let right = self.parse_expression(self.get_precedence_value("PREFIX"))?;
        Ok(Box::new(ast::PrefixExpression {
            token: curr_token,
            operator: operator,
            right: right,
        }))
    }

    fn parse_block_statement(&mut self) -> Result<Box<BlockStatement>, ParseError> {
        let current_token = self.get_current_token()?;
        let mut stmts: Vec<Box<dyn Statement>> = vec![];
        loop {
            self.skip_current_semicolon_token();
            let current_token_type = self.get_current_token()?.token_type;
            if current_token_type == TokenType::RBRACE || current_token_type == TokenType::EOF {
                self.next_token();
                break;
            }
            let stmt = self.parse_statement()?;
            stmts.push(stmt);
        }
        Ok(Box::new(BlockStatement {
            token: current_token,
            statements: stmts,
        }))
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

    fn get_current_token(&self) -> Result<Token, ParseError> {
        let token = match self.curr_token.as_ref() {
            Some(tok) => Ok(tok.clone()),
            None => Err(ParseError::new(
                String::from("Error getting current token"),
                None,
                None,
            )),
        };
        token
    }

    fn check_current_token_match(&self, token_type: TokenType) -> bool {
        match self.curr_token.as_ref() {
            Some(tok) => {
                if tok.token_type == token_type {
                    return true;
                }
            }
            None => return false,
        };
        return false;
    }

    fn get_current_token_and_skip(&mut self) -> Result<Token, ParseError> {
        let mut found = false;
        let token = match self.curr_token.as_ref() {
            Some(tok) => {
                found = true;
                Ok(tok.clone())
            }
            None => Err(ParseError::new(
                String::from("Error getting current token"),
                None,
                None,
            )),
        };
        if found {
            self.next_token();
        }
        token
    }

    fn get_peek_token(&self) -> Option<Token> {
        let peek_token = match self.peek_token.as_ref() {
            Some(token) => Some(token.clone()),
            None => None,
        };
        peek_token
    }

    fn current_precedence(&self) -> Result<PrecedenceValue, ParseError> {
        let curr_token = self.get_current_token()?;
        let p: PrecedenceValue;
        match self.precedences.get(&curr_token.token_type) {
            Some(precedence) => {
                p = *precedence;
            }
            None => {
                p = self.get_precedence_value("LOWEST");
            }
        }
        return Ok(p);
    }

    fn peek_precedence(&self) -> Result<PrecedenceValue, ParseError> {
        match self.get_peek_token() {
            Some(peek_token) => {
                let p: PrecedenceValue;
                match self.precedences.get(&peek_token.token_type) {
                    Some(precedence) => {
                        p = *precedence;
                    }
                    None => {
                        p = self.get_precedence_value("LOWEST");
                    }
                }
                return Ok(p);
            }
            None => {
                return Err(ParseError::new(
                    String::from("Couldn't get peek precedence"),
                    None,
                    Some(self.get_current_token()?.line),
                ))
            }
        }
    }

    #[allow(dead_code)]
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

    fn expect_peek_token_with_type(&mut self, token_type: TokenType) -> bool {
        let flag = self.check_peek_token_match(token_type);
        if flag {
            self.next_token();
        }
        flag
    }

    fn expect_peek_token_with_type_and_no_advance(&mut self, token_type: TokenType) -> bool {
        let flag = self.check_peek_token_match(token_type);
        flag
    }

    fn skip_peek_token_if_match(&mut self, token_type: TokenType) {
        if self.check_peek_token_match(token_type) {
            self.next_token();
        }
    }

    fn check_peek_token_match(&mut self, token_type: TokenType) -> bool {
        match self.peek_token.as_ref() {
            Some(token) => {
                if token.token_type == token_type {
                    return true;
                }
            }
            None => {}
        };
        return false;
    }

    fn skip_peek_semicolon_token(&mut self) {
        if self.check_peek_token_match(TokenType::SEMICOLON) {
            self.next_token();
        }
    }

    fn skip_current_semicolon_token(&mut self) {
        if self.check_current_token_match(TokenType::SEMICOLON) {
            self.next_token();
        }
    }
}
