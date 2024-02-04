use bolt::{
    error::BoltError,
    evaluator::{environment::Environment, evaluator::Evaluator},
};
use std::io::{self, Write};

pub fn start_repl() -> io::Result<()> {
    let mut user_input = String::new();
    let stdin = io::stdin();
    let environment = Environment::new();
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
        let evaluator = Evaluator::new(user_input.clone(), Some(environment.clone()));
        match evaluator.eval() {
            Some(evaluated) => match evaluated {
                Ok(result) => {
                    println!("{}", result.inspect());
                }
                Err(e) => {
                    panic!("{}", e.get_message());
                }
            },
            None => {
                panic!("Something went wrong!");
            }
        }
        user_input.clear();
    }
}
