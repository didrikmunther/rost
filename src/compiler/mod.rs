use crate::parser::Declaration;

use self::{code::Code, program::Program, error::CompilerError};

mod asm;
mod code;
mod program;
mod error;
mod row;
mod system_call;

pub fn compile(parsed: &Vec<Declaration>) -> Result<Code, CompilerError> {
    Program::new().compile(parsed)?.asm().generate_code()
}
