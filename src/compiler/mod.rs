use crate::parser::definition::Declaration;

use self::{error::CompilerError, program::Program};

pub mod builder;
pub mod definition;
pub mod program;

mod assignment;
mod declaration;
mod error;
mod expression;
mod function_call;
mod function_declaration;
mod if_statement;
mod util;
mod while_statement;

pub fn compile(parsed: &Vec<Declaration>) -> Result<Program, CompilerError> {
    Program::new().compile(parsed)
}
