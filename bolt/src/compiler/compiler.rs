use crate::parser::ast::{Program, Statement};

use super::LLVM;

pub trait Compiler {
    fn compile(&mut self) -> &mut Self;
    fn clean(&mut self);
    fn compile_statement(&mut self, statement: &Box<dyn Statement>);
    fn generate_ir(&mut self) -> String;
    fn ir_to_file(&mut self, filename: String);
    fn bytecode_to_file(&mut self, filename: String, target: &String);
}

#[derive(Clone, Copy)]
pub enum CompilerBackend {
    LLVM,
}

pub struct Factory {}

impl Factory {
    pub fn new(backend: CompilerBackend, program: Program) -> impl Compiler {
        match backend {
            CompilerBackend::LLVM => unsafe { LLVM::new(program) },
        }
    }
}
