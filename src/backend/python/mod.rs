use generator::Generator;

use super::Backend;
use crate::{compiler::program::Program, error::RostError};

mod code;
mod error;
mod generator;

pub struct PythonBackend;

impl Backend for PythonBackend {
    fn generate(program: &Program) -> Result<String, RostError> {
        Ok(Generator.generate_code(program)?)
    }
}
