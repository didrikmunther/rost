use std::collections::HashMap;

use super::error::CompilerError;
use crate::parser::definition::Declaration;
use builder::Builder;
use function_declaration::Function;
use ir::{PrimitiveValue, Variable, VariableId};

mod assignment;
mod builder;
mod declaration;
mod expression;
mod function_call;
mod function_declaration;
pub mod ir;
mod util;
mod variable;

#[derive(Debug, Default)]
pub struct Program {
    pub instructions: Builder,
    pub variables: Vec<Variable>,
    pub variable_lookup: HashMap<String, VariableId>,
    pub global_data: Vec<PrimitiveValue>,
    pub functions: Vec<Function>,

    // How many parameters does the main function take?
    pub main_func_nparams: usize,
}

impl Program {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn compile(mut self, parsed: Vec<Declaration>) -> Result<Program, CompilerError> {
        self.instructions = self.get_instructions(&parsed)?;

        Ok(self)
    }
}
