use bolt::evaluator::{environment::Environment, evaluator::eval};

use crate::repl;

use std::fs;

pub fn start() {
    repl::start_repl().unwrap();
}

pub fn run(path: String) {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let mut environment = Environment::new();
    let evaluated = eval(contents, &mut environment).unwrap();
    println!("{}", evaluated.inspect());
}
