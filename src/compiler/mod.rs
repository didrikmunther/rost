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
mod return_statement;
mod scope;
mod util;
mod while_statement;

pub fn compile<'a>(parsed: Vec<Declaration>) -> Result<Program, CompilerError> {
    Program::new().compile(parsed)
}
