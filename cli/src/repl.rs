#![allow(dead_code)]
use bolt::{evaluator::evaluator::evaluate_statement, parser::parser};
use std::io::{self, Write};

pub fn start_repl() -> io::Result<()> {
    let mut user_input = String::new();
    let stdin = io::stdin();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
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
            for stmt in p.stmts {
                let e = evaluate_statement(&stmt).unwrap();
                println!("{}", e.inspect());
            }
        }
        user_input.clear();
    }
}
