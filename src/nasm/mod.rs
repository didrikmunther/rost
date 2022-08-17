use crate::compiler::program::Program;

use self::{code::Code, error::NasmError, generator::Generator};

mod code;
mod error;
mod generator;
mod row;
mod system_call;

pub fn generate(program: &Program) -> Result<Code, NasmError> {
    Generator::new(program).generate_code()
}