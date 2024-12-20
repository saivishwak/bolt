use super::token;
use regex::Regex;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    input_chars: Vec<char>,
    pub position: usize, // current position in input (points to current char)
    pub read_position: usize, // current reading position in input (after current char)
    curr_line: usize,
    ch: char, // current char under examination
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        let mut lexer = Self {
            input: source,
            input_chars: source.chars().collect(), //Make the source into vector of chars
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
                    // Loop until you find the new line to exit the comment
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
                    let token = token::lookup_indentifier(literal);
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
        //Adding '.' for float
        while self.is_digit(self.ch) || self.ch == '.' {
            self.read_char();
        }
        return &self.input[position..self.position];
    }

    fn is_at_end(&self) -> bool {
        return self.position == self.input.len();
    }
}
