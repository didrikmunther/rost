use std::collections::HashMap;

use super::error::CompilerError;
use crate::parser::definition::Declaration;
use builder::Builder;
use ir::{PrimitiveValue, Variable, VariableId, VariableScope};

mod assignment;
pub mod builder;
mod declaration;
mod expression;
mod function_call;
mod function_declaration;
pub mod ir;
mod util;
mod variable;

#[derive(Debug, Default)]
pub struct Scope {
    pub variable_lookup: HashMap<String, VariableId>,
}

impl Scope {
    // Only keep global variables
    pub fn from_parent(parent: &Scope, variables: &[Variable]) -> Self {
        Self {
            variable_lookup: parent
                .variable_lookup
                .clone()
                .into_iter()
                .filter(|(_, variable_id)| {
                    variables.get(*variable_id).unwrap().scope == VariableScope::Global
                })
                .collect::<HashMap<_, _>>(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Program {
    pub instructions: Builder,
    pub variables: Vec<Variable>,
    pub global_data: Vec<PrimitiveValue>,

    pub scope: Scope,

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
