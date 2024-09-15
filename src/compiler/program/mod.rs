use super::error::CompilerError;
use crate::parser::definition::Declaration;
use builder::Builder;
use ir::Variable;

mod assignment;
mod builder;
mod declaration;
mod expression;
mod ir;
mod util;
mod variable;

#[derive(Debug, Default)]
pub struct Program {
    pub procedures: Builder,
    pub variables: Vec<Variable>,

    // How many parameters does the main function take?
    pub main_func_nparams: usize,
}

impl Program {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn compile(mut self, parsed: Vec<Declaration>) -> Result<Program, CompilerError> {
        self.procedures = self.get_procedures(&parsed)?;

        Ok(self)
    }
}
