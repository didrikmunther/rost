use super::Backend;
use crate::{compiler::program::Program, error::RostError};
use generator::Generator;

mod code;
mod error;
mod generator;

pub struct PythonBackend;

impl Backend for PythonBackend {
    fn generate(&mut self, program: &Program) -> Result<String, RostError> {
        Ok(Generator.generate_code(program)?)
    }
}
