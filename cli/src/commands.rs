use crate::repl;
use bolt::{error::BoltError, evaluator::evaluator::Evaluator};
use std::fs;

pub fn start() {
    repl::start_repl().unwrap();
}

pub fn run(path: String) {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let evaluator = Evaluator::new(contents, None);
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
}
