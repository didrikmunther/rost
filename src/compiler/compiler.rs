use crate::parser::Declaration;

use super::{program::Program, error::CompilerError};

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(&mut self, parsed: &Vec<Declaration>) -> Result<Program, CompilerError> {
        Ok(Program::new())
    }
}