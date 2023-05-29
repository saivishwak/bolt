#![allow(dead_code)]
use super::token;
use regex::Regex;

const KEYWORDS: [&str; 2] = ["let", "fn"];

pub struct Lexer<'a> {
    input: &'a str,
    input_chars: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        let mut lexer = Self {
            input: source,
            input_chars: source.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        return lexer;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = *self.input_chars.get(self.read_position).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> token::Token {
        let tok: token::Token;
        self.skip_whitespace();
        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = token::Token {
                        token_type: token::TokenType::EQ,
                        literal: String::from("=="),
                    };
                } else {
                    tok = token::Token {
                        token_type: token::TokenType::ASSIGN,
                        literal: String::from(self.ch),
                    };
                }
            }
            '+' => {
                tok = token::Token {
                    token_type: token::TokenType::PLUS,
                    literal: String::from(self.ch),
                };
            }
            '-' => {
                tok = token::Token {
                    token_type: token::TokenType::MINUS,
                    literal: String::from(self.ch),
                };
            }
            ';' => {
                tok = token::Token {
                    token_type: token::TokenType::SEMICOLON,
                    literal: String::from(self.ch),
                };
            }
            '(' => {
                tok = token::Token {
                    token_type: token::TokenType::LPAREN,
                    literal: String::from(self.ch),
                };
            }
            ')' => {
                tok = token::Token {
                    token_type: token::TokenType::RPAREN,
                    literal: String::from(self.ch),
                };
            }
            '{' => {
                tok = token::Token {
                    token_type: token::TokenType::LBRACE,
                    literal: String::from(self.ch),
                };
            }
            '}' => {
                tok = token::Token {
                    token_type: token::TokenType::RBRACE,
                    literal: String::from(self.ch),
                };
            }
            ',' => {
                tok = token::Token {
                    token_type: token::TokenType::COMMA,
                    literal: String::from(self.ch),
                };
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = token::Token {
                        token_type: token::TokenType::NOTEQ,
                        literal: String::from("!="),
                    };
                } else {
                    tok = token::Token {
                        token_type: token::TokenType::BANG,
                        literal: String::from(self.ch),
                    };
                }
            }
            '/' => {
                tok = token::Token {
                    token_type: token::TokenType::SLASH,
                    literal: String::from(self.ch),
                };
            }
            '*' => {
                tok = token::Token {
                    token_type: token::TokenType::ASTERISK,
                    literal: String::from(self.ch),
                };
            }
            '<' => {
                tok = token::Token {
                    token_type: token::TokenType::LT,
                    literal: String::from(self.ch),
                };
            }
            '>' => {
                tok = token::Token {
                    token_type: token::TokenType::GT,
                    literal: String::from(self.ch),
                };
            }
            '\0' => {
                tok = token::Token {
                    token_type: token::TokenType::EOF,
                    literal: String::from(self.ch),
                };
            }
            _ => {
                // Find if its a letter or digit
                if self.is_letter(self.ch) {
                    let literal = self.lookup_identifier();
                    let token = token::lookup_indent(literal);
                    tok = token::Token {
                        token_type: token,
                        literal: String::from(literal),
                    };
                    return tok;
                } else if self.is_digit(self.ch) {
                    let literal = self.read_number();
                    tok = token::Token {
                        token_type: token::TokenType::INT,
                        literal: String::from(literal),
                    };
                    return tok;
                } else {
                    tok = token::Token {
                        token_type: token::TokenType::ILLIGAL,
                        literal: String::from('\0'),
                    }
                }
            }
        }
        self.read_char();
        tok
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn is_letter(&self, ch: char) -> bool {
        //Allowing _ as words can have _ in them
        let letter_pattern = Regex::new(r"[a-zA-Z_]").unwrap();
        if letter_pattern.is_match(&ch.to_string()) {
            return true;
        }
        false
    }

    fn is_digit(&mut self, ch: char) -> bool {
        let letter_pattern = Regex::new(r"[0-9]").unwrap();
        if letter_pattern.is_match(&ch.to_string()) {
            return true;
        }
        false
    }

    fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        } else {
            return *self.input_chars.get(self.read_position).unwrap();
        }
    }

    fn lookup_identifier(&mut self) -> &str {
        let position = self.position;
        while self.is_letter(self.ch) {
            self.read_char();
        }
        return &self.input[position..self.position];
    }

    fn read_number(&mut self) -> &str {
        let position = self.position;
        while self.is_digit(self.ch) {
            self.read_char();
        }
        return &self.input[position..self.position];
    }
}

#[cfg(test)]
mod lexer_test {
    use crate::token::{Token, TokenType};

    #[test]
    fn next_token() {
        let mut lexer = super::Lexer::new(
            "let five = (5);
        let ten = 10;
        let add = fn(x, y) {
        x + y;
        };
        let final_result = add(five, ten);
        
        !-/*5;
        5 < 10 > 5;
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        10 == 10;
        10 != 9;",
        );
        let mut tokens: Vec<Token> = Vec::new();
        let test_tokens = vec![
            Token {
                token_type: TokenType::LET,
                literal: String::from("let"),
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("five"),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: String::from("("),
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("5"),
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: String::from(")"),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::LET,
                literal: String::from("let"),
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("ten"),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("10"),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::LET,
                literal: String::from("let"),
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("add"),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::FUNCTION,
                literal: String::from("fn"),
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: String::from("("),
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("x"),
            },
            Token {
                token_type: TokenType::COMMA,
                literal: String::from(","),
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("y"),
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: String::from(")"),
            },
            Token {
                token_type: TokenType::LBRACE,
                literal: String::from("{"),
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("x"),
            },
            Token {
                token_type: TokenType::PLUS,
                literal: String::from("+"),
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("y"),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::RBRACE,
                literal: String::from("}"),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::LET,
                literal: String::from("let"),
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("final_result"),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("add"),
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: String::from("("),
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("five"),
            },
            Token {
                token_type: TokenType::COMMA,
                literal: String::from(","),
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("ten"),
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: String::from(")"),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::BANG,
                literal: String::from("!"),
            },
            Token {
                token_type: TokenType::MINUS,
                literal: String::from("-"),
            },
            Token {
                token_type: TokenType::SLASH,
                literal: String::from("/"),
            },
            Token {
                token_type: TokenType::ASTERISK,
                literal: String::from("*"),
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("5"),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("5"),
            },
            Token {
                token_type: TokenType::LT,
                literal: String::from("<"),
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("10"),
            },
            Token {
                token_type: TokenType::GT,
                literal: String::from(">"),
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("5"),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::IF,
                literal: String::from("if"),
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: String::from("("),
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("5"),
            },
            Token {
                token_type: TokenType::LT,
                literal: String::from("<"),
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("10"),
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: String::from(")"),
            },
            Token {
                token_type: TokenType::LBRACE,
                literal: String::from("{"),
            },
            Token {
                token_type: TokenType::RETURN,
                literal: String::from("return"),
            },
            Token {
                token_type: TokenType::TRUE,
                literal: String::from("true"),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::RBRACE,
                literal: String::from("}"),
            },
            Token {
                token_type: TokenType::ELSE,
                literal: String::from("else"),
            },
            Token {
                token_type: TokenType::LBRACE,
                literal: String::from("{"),
            },
            Token {
                token_type: TokenType::RETURN,
                literal: String::from("return"),
            },
            Token {
                token_type: TokenType::FALSE,
                literal: String::from("false"),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::RBRACE,
                literal: String::from("}"),
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("10"),
            },
            Token {
                token_type: TokenType::EQ,
                literal: String::from("=="),
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("10"),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("10"),
            },
            Token {
                token_type: TokenType::NOTEQ,
                literal: String::from("!="),
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("9"),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
            },
        ];
        loop {
            let token = lexer.next_token();
            if token.token_type == TokenType::EOF {
                break;
            }
            tokens.push(token);
        }
        assert_eq!(tokens, test_tokens);
    }
}
