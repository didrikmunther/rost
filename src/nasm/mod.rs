use crate::compiler::program::Program;

use self::{code::Code, error::NasmError, generator::Generator};

pub mod code;
mod error;
mod generator;
mod row;
mod system_call;

pub fn generate(
    program: &Program,
    with_comments: bool,
    with_optimization: bool,
) -> Result<Code, NasmError> {
    Generator::new(program)
        .with_comments(with_comments)
        .with_optimization(with_optimization)
        .generate_code()
}
