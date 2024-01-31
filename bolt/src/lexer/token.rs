#![allow(dead_code)]
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum TokenType {
    EOF,
    ILLIGAL,

    //Identifiers
    IDENTIFIER,
    STRING,
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
    LTEQ,
    GTEQ,

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
    NULL,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

/*
When resolving indentifier check if the indentifier is a reserved word
*/
pub fn lookup_indentifier(indent: &str) -> TokenType {
    let keywords: HashMap<&str, TokenType> = HashMap::from([
        ("let", TokenType::LET),
        ("null", TokenType::NULL),
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

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub line: usize,
}
