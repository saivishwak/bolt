use crate::repl;
use bolt::{
    error::BoltError, evaluator::evaluator::Evaluator, parser::parser::Parser, Compiler,
    CompilerBackend, Factory,
};
use std::fs;

pub fn start() {
    repl::start_repl().unwrap();
}

pub fn run(path: &String) {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let evaluator = Evaluator::new(contents, None, false, None);
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

pub fn jit(path: &String, backend: &CompilerBackend) {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let evaluator = Evaluator::new(contents, None, true, Some(backend.clone()));
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

pub fn compile(
    path: &String,
    backend: &CompilerBackend,
    out: String,
    target: &String,
    bytecode: bool,
) {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let source = contents;

    let mut parser = Parser::new(&source);
    match parser.parse_program() {
        Ok(program) => {
            let mut compiler = Factory::new(*backend, program);
            let compile_string = compiler.compile();
            if bytecode {
                println!("Compiling to bytecode");
                compile_string.bytecode_to_file(out, target)
            } else {
                compile_string.ir_to_file(out);
            }
        }
        Err(e) => {
            panic!("Error compiling")
        }
    }
}
