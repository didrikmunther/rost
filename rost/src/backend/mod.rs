use crate::{compiler::program::Program, error::RostError};

pub mod python;
pub mod wasm;

pub trait Backend {
    fn generate(&mut self, program: &Program) -> Result<String, RostError>;
}
