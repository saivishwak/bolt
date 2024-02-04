use bolt::{
    error::BoltError,
    evaluator::{environment::Environment, evaluator::eval},
};

use crate::repl;

use std::fs;

pub fn start() {
    repl::start_repl().unwrap();
}

pub fn run(path: String) {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let mut environment = Environment::new();
    match eval(contents, &mut environment) {
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
}
