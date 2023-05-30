#![allow(dead_code)]
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    EOF,
    ILLIGAL,

    //Identifiers
    IDENTIFIER,
    INT,

    //Operators
    ASSIGN,
    EQ,
    NOTEQ,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,

    //Delimeters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    //Keyworks
    LET,
    FUNCTION,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

pub fn lookup_indent(indent: &str) -> TokenType {
    let keywords: HashMap<&str, TokenType> = HashMap::from([
        ("let", TokenType::LET),
        ("fn", TokenType::FUNCTION),
        ("if", TokenType::IF),
        ("else", TokenType::ELSE),
        ("return", TokenType::RETURN),
        ("true", TokenType::TRUE),
        ("false", TokenType::FALSE),
    ]);

    match keywords.get(indent) {
        Some(token) => {
            return token.clone();
        }
        None => {
            //
        }
    };

    return TokenType::IDENTIFIER;
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub line: usize,
}
