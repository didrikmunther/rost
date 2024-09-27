use super::Backend;
use crate::{compiler::program::Program, error::RostError};
use generator::Generator;

mod generator;
mod code;
mod error;

pub struct WasmBackend;

impl Backend for WasmBackend {
    fn generate(&mut self, program: &Program) -> Result<String, RostError> {
        Ok(Generator.generate_code(program)?)
    }
}
