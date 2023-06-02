#![allow(dead_code)]
use super::parser;
use crate::lexer;
use crate::token;
use std::io;

pub fn start_repl() -> io::Result<()> {
    let mut user_input = String::new();
    let stdin = io::stdin();

    loop {
        let _read_ok = match stdin.read_line(&mut user_input) {
            Ok(_input) => {
                //
            }
            Err(err) => {
                return Err(err);
            }
        };
        let mut l = lexer::Lexer::new(&user_input);
        let mut parser = parser::Parser::new(&user_input);
        /*loop {
            let token = l.next_token();
            if token.token_type == token::TokenType::EOF {
                break;
            }
            println!("{:#?}", token);
        }*/
        let p = parser.parse_program();
        println!("{:#?}", p.stmts);
    }
}
