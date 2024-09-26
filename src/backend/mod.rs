use crate::{compiler::program::Program, error::RostError};

pub mod python;

pub trait Backend {
    fn generate(program: &Program) -> Result<String, RostError>;
}
