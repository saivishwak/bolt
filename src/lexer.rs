#![allow(dead_code)]
use super::token;
use regex::Regex;

pub struct Lexer<'a> {
    input: &'a str,
    input_chars: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    curr_line: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        let mut lexer = Self {
            input: source,
            input_chars: source.chars().collect(),
            position: 0,
            curr_line: 0,
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

        if self.ch == '\n' {
            self.curr_line += 1;
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn create_new_token(
        &self,
        t_type: token::TokenType,
        literal: String,
        line: usize,
    ) -> token::Token {
        return token::Token {
            token_type: t_type,
            literal: literal,
            line: line,
        };
    }

    pub fn get_tokens(&mut self) -> Vec<token::Token> {
        let mut tokens: Vec<token::Token> = vec![];
        loop {
            let token = self.next_token();
            let token_type = token.token_type;
            tokens.push(token);
            if token_type == token::TokenType::EOF {
                break;
            }
        }
        tokens
    }

    pub fn next_token(&mut self) -> token::Token {
        let tok: token::Token;
        self.skip_whitespace();
        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = self.create_new_token(
                        token::TokenType::EQ,
                        String::from("=="),
                        self.curr_line,
                    );
                } else {
                    tok = self.create_new_token(
                        token::TokenType::ASSIGN,
                        String::from(self.ch),
                        self.curr_line,
                    );
                }
            }
            '+' => {
                tok = self.create_new_token(
                    token::TokenType::PLUS,
                    String::from(self.ch),
                    self.curr_line,
                );
            }
            '-' => {
                tok = self.create_new_token(
                    token::TokenType::MINUS,
                    String::from(self.ch),
                    self.curr_line,
                );
            }
            ';' => {
                tok = self.create_new_token(
                    token::TokenType::SEMICOLON,
                    String::from(self.ch),
                    self.curr_line,
                );
            }
            '(' => {
                tok = self.create_new_token(
                    token::TokenType::LPAREN,
                    String::from(self.ch),
                    self.curr_line,
                );
            }
            ')' => {
                tok = self.create_new_token(
                    token::TokenType::RPAREN,
                    String::from(self.ch),
                    self.curr_line,
                );
            }
            '{' => {
                tok = self.create_new_token(
                    token::TokenType::LBRACE,
                    String::from(self.ch),
                    self.curr_line,
                );
            }
            '}' => {
                tok = self.create_new_token(
                    token::TokenType::RBRACE,
                    String::from(self.ch),
                    self.curr_line,
                );
            }
            ',' => {
                tok = self.create_new_token(
                    token::TokenType::COMMA,
                    String::from(self.ch),
                    self.curr_line,
                );
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = self.create_new_token(
                        token::TokenType::NOTEQ,
                        String::from("!="),
                        self.curr_line,
                    );
                } else {
                    tok = self.create_new_token(
                        token::TokenType::BANG,
                        String::from(self.ch),
                        self.curr_line,
                    );
                }
            }
            '/' => {
                // Case for comments which needs to be ignored
                if self.peek_char() == '/' {
                    loop {
                        self.read_char();
                        if self.ch == '\n' {
                            break;
                        }
                    }
                    return self.next_token();
                } else {
                    tok = self.create_new_token(
                        token::TokenType::SLASH,
                        String::from(self.ch),
                        self.curr_line,
                    );
                }
            }
            '*' => {
                tok = self.create_new_token(
                    token::TokenType::ASTERISK,
                    String::from(self.ch),
                    self.curr_line,
                );
            }
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = self.create_new_token(
                        token::TokenType::LTEQ,
                        String::from("<="),
                        self.curr_line,
                    );
                } else {
                    tok = self.create_new_token(
                        token::TokenType::LT,
                        String::from(self.ch),
                        self.curr_line,
                    );
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = self.create_new_token(
                        token::TokenType::GTEQ,
                        String::from(">="),
                        self.curr_line,
                    );
                } else {
                    tok = self.create_new_token(
                        token::TokenType::GT,
                        String::from(self.ch),
                        self.curr_line,
                    );
                }
            }
            '\0' => {
                tok = self.create_new_token(
                    token::TokenType::EOF,
                    String::from(self.ch),
                    self.curr_line,
                );
            }
            '"' => {
                let start = self.read_position;
                while self.peek_char() != '"' && !self.is_at_end() {
                    if self.ch == '\n' {
                        self.curr_line += 1;
                    }
                    self.read_char();
                }
                //advance to move next to quote
                self.read_char();
                let value = &self.input[start..self.position];
                tok = self.create_new_token(
                    token::TokenType::STRING,
                    String::from(value),
                    self.curr_line,
                );
            }
            _ => {
                // Find if its a letter or digit
                if self.is_letter(self.ch) {
                    let literal = self.lookup_identifier();
                    let token = token::lookup_indent(literal);
                    tok = token::Token {
                        token_type: token,
                        literal: String::from(literal),
                        line: self.curr_line,
                    };
                    return tok;
                } else if self.is_digit(self.ch) {
                    let literal = self.read_number();
                    tok = token::Token {
                        token_type: token::TokenType::INT,
                        literal: String::from(literal),
                        line: self.curr_line,
                    };
                    return tok;
                } else {
                    tok = token::Token {
                        token_type: token::TokenType::ILLIGAL,
                        literal: String::from('\0'),
                        line: self.curr_line,
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

    fn is_at_end(&self) -> bool {
        return self.position == self.input.len();
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
        10 != 9;
        //comment
        12;",
        );
        let mut tokens: Vec<Token> = Vec::new();
        let test_tokens = vec![
            Token {
                token_type: TokenType::LET,
                literal: String::from("let"),
                line: 0,
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("five"),
                line: 0,
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: String::from("="),
                line: 0,
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: String::from("("),
                line: 0,
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("5"),
                line: 0,
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: String::from(")"),
                line: 0,
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
                line: 0,
            },
            Token {
                token_type: TokenType::LET,
                literal: String::from("let"),
                line: 1,
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("ten"),
                line: 1,
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: String::from("="),
                line: 1,
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("10"),
                line: 1,
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
                line: 1,
            },
            Token {
                token_type: TokenType::LET,
                literal: String::from("let"),
                line: 2,
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("add"),
                line: 2,
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: String::from("="),
                line: 2,
            },
            Token {
                token_type: TokenType::FUNCTION,
                literal: String::from("fn"),
                line: 2,
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: String::from("("),
                line: 2,
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("x"),
                line: 2,
            },
            Token {
                token_type: TokenType::COMMA,
                literal: String::from(","),
                line: 2,
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("y"),
                line: 2,
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: String::from(")"),
                line: 2,
            },
            Token {
                token_type: TokenType::LBRACE,
                literal: String::from("{"),
                line: 2,
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("x"),
                line: 3,
            },
            Token {
                token_type: TokenType::PLUS,
                literal: String::from("+"),
                line: 3,
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("y"),
                line: 3,
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
                line: 3,
            },
            Token {
                token_type: TokenType::RBRACE,
                literal: String::from("}"),
                line: 4,
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
                line: 4,
            },
            Token {
                token_type: TokenType::LET,
                literal: String::from("let"),
                line: 5,
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("final_result"),
                line: 5,
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: String::from("="),
                line: 5,
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("add"),
                line: 5,
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: String::from("("),
                line: 5,
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("five"),
                line: 5,
            },
            Token {
                token_type: TokenType::COMMA,
                literal: String::from(","),
                line: 5,
            },
            Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("ten"),
                line: 5,
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: String::from(")"),
                line: 5,
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
                line: 5,
            },
            Token {
                token_type: TokenType::BANG,
                literal: String::from("!"),
                line: 6,
            },
            Token {
                token_type: TokenType::MINUS,
                literal: String::from("-"),
                line: 6,
            },
            Token {
                token_type: TokenType::SLASH,
                literal: String::from("/"),
                line: 6,
            },
            Token {
                token_type: TokenType::ASTERISK,
                literal: String::from("*"),
                line: 6,
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("5"),
                line: 6,
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
                line: 6,
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("5"),
                line: 7,
            },
            Token {
                token_type: TokenType::LT,
                literal: String::from("<"),
                line: 7,
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("10"),
                line: 7,
            },
            Token {
                token_type: TokenType::GT,
                literal: String::from(">"),
                line: 7,
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("5"),
                line: 7,
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
                line: 7,
            },
            Token {
                token_type: TokenType::IF,
                literal: String::from("if"),
                line: 8,
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: String::from("("),
                line: 8,
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("5"),
                line: 8,
            },
            Token {
                token_type: TokenType::LT,
                literal: String::from("<"),
                line: 8,
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("10"),
                line: 8,
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: String::from(")"),
                line: 8,
            },
            Token {
                token_type: TokenType::LBRACE,
                literal: String::from("{"),
                line: 8,
            },
            Token {
                token_type: TokenType::RETURN,
                literal: String::from("return"),
                line: 9,
            },
            Token {
                token_type: TokenType::TRUE,
                literal: String::from("true"),
                line: 9,
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
                line: 9,
            },
            Token {
                token_type: TokenType::RBRACE,
                literal: String::from("}"),
                line: 10,
            },
            Token {
                token_type: TokenType::ELSE,
                literal: String::from("else"),
                line: 10,
            },
            Token {
                token_type: TokenType::LBRACE,
                literal: String::from("{"),
                line: 10,
            },
            Token {
                token_type: TokenType::RETURN,
                literal: String::from("return"),
                line: 11,
            },
            Token {
                token_type: TokenType::FALSE,
                literal: String::from("false"),
                line: 11,
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
                line: 11,
            },
            Token {
                token_type: TokenType::RBRACE,
                literal: String::from("}"),
                line: 12,
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("10"),
                line: 13,
            },
            Token {
                token_type: TokenType::EQ,
                literal: String::from("=="),
                line: 13,
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("10"),
                line: 13,
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
                line: 13,
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("10"),
                line: 14,
            },
            Token {
                token_type: TokenType::NOTEQ,
                literal: String::from("!="),
                line: 14,
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("9"),
                line: 14,
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
                line: 14,
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("12"),
                line: 16,
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
                line: 16,
            },
        ];
        loop {
            let token = lexer.next_token();
            if token.token_type == TokenType::EOF {
                break;
            }
            println!("{:#?}", token);
            tokens.push(token);
        }
        assert_eq!(tokens, test_tokens);
    }
}
