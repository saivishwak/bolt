#![allow(dead_code)]
use crate::parser::parser;
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
        let mut parser = parser::Parser::new(&user_input);
        if let Ok(p) = parser.parse_program() {
            println!("{:#?}", p.stmts);
        }
        user_input.clear();
    }
}
