use crate::parser::definition::Declaration;

use self::{error::CompilerError, program::Program};

pub mod error;
pub mod program;

pub fn compile(parsed: Vec<Declaration>) -> Result<Program, CompilerError> {
    Program::new().compile(parsed)
}
