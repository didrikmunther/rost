use crate::compiler::program::Program;

use self::{code::Code, error::NasmError, generator::Generator};

pub mod code;

mod arithmetic;
mod assign;
mod data;
mod error;
mod function_call;
mod function_declaration;
mod function_return;
mod generator;
mod if_statement;
mod push;
mod row;
mod while_statement;

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
