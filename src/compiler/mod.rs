use crate::parser::Declaration;

use self::{code::Code, compiler::Compiler, error::CompilerError};

mod asm;
mod code;
mod compiler;
mod error;
mod program;
mod row;

pub fn compile(parsed: &Vec<Declaration>) -> Result<Code, CompilerError> {
    Compiler::new().compile(parsed)?.asm().generate_code()
}
