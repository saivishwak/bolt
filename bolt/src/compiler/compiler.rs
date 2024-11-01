use crate::parser::ast::Program;

use super::LLVM;

pub trait Compiler {
    fn compile(&mut self) -> &mut Self;
    fn clean(&mut self);
    fn generate_ir(&mut self) -> String;
    fn ir_to_file(&mut self, filename: String);
    fn bytecode_to_file(&mut self, filename: String, target: &String);
    fn bytecode_to_jit(&mut self, target: &String);
}

#[derive(Clone, Copy)]
pub enum CompilerBackend {
    LLVM,
}

pub struct Factory {}

impl Factory {
    pub fn new(backend: CompilerBackend, program: Program, filename: &str) -> impl Compiler {
        match backend {
            CompilerBackend::LLVM => LLVM::new(program, filename),
        }
    }
}
