use crate::parser::definition::Declaration;

use self::{error::CompilerError, program::Program};

mod builder;
mod declaration;
pub mod definition;
mod error;
mod expression;
mod function_call;
pub mod program;
mod assignment;
mod function_declaration;

pub fn compile(parsed: &Vec<Declaration>) -> Result<Program, CompilerError> {
    Program::new().compile(parsed)
}
