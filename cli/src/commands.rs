use crate::repl;
use bolt::{
    error::BoltError, evaluator::evaluator::Evaluator, parser::parser::Parser, Compiler,
    CompilerBackend, Factory,
};
use std::fs;

use regex::Regex;

fn extract_filename(filepath: &str) -> Option<String> {
    // Regex pattern to match the filename in a filepath
    let re = Regex::new(r"([^/\\]+)\.[^.]+$").unwrap();

    // Find the match and return the filename
    if let Some(captures) = re.captures(filepath) {
        captures.get(1).map(|m| m.as_str().to_string())
    } else {
        None
    }
}

pub fn start() {
    repl::start_repl().unwrap();
}

pub fn run(path: &String) {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let evaluator = Evaluator::new(&contents, "evaluator", None, false, None, "");
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
    let filename = extract_filename(&path).expect("Improper filename");
    let evaluator = Evaluator::new(
        &contents,
        &filename,
        None,
        true,
        Some(backend.clone()),
        "x86_64-pc-linux-gnu",
    );
    match evaluator.eval() {
        Some(evaluated) => match evaluated {
            Ok(_) => {}
            Err(e) => {
                panic!("{}", e.get_message());
            }
        },
        None => {}
    }
}

pub fn compile(
    path: &String,
    backend: &CompilerBackend,
    out: String,
    target: &String,
    bytecode: bool,
) {
    let contents = fs::read_to_string(path).expect("Error Reading the source file!");
    let mut parser = Parser::new(&contents);
    let filename = extract_filename(&path).expect("Improper filename");
    match parser.parse_program() {
        Ok(program) => {
            let mut compiler = Factory::new(*backend, program, &filename);
            compiler.compile();
            if bytecode {
                println!("Compiling to bytecode");
                compiler.bytecode_to_file(out, target)
            } else {
                compiler.ir_to_file(out);
            }
        }
        Err(e) => {
            panic!("Error compiling {}", e.get_message())
        }
    }
}
