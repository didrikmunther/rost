use std::collections::HashMap;

use super::error::CompilerError;
use crate::parser::definition::Declaration;
use builder::Builder;
use ir::{PrimitiveValue, Variable};
use scope::{Scope, ScopeId};

mod assignment;
pub mod builder;
mod declaration;
mod expression;
mod function_call;
mod function_declaration;
pub mod ir;
mod scope;
mod util;
mod variable;

pub type Location = usize;

#[derive(Debug, Default)]
pub struct Program {
    pub instructions: Builder,
    pub variables: Vec<Variable>,
    pub global_data: Vec<PrimitiveValue>,

    pub scopes: Vec<Scope>,
    pub scope_id: ScopeId,
    // This is quite a hack, but it's the easiest way to get the scope of a variable.
    // We should use some kind of run-length encoding to make this more efficient.
    pub scope_lookup: HashMap<Location, ScopeId>,

    // How many parameters does the main function take?
    pub main_func_nparams: usize,
}

impl Program {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::default()],
            scope_id: 0,
            ..Default::default()
        }
    }

    pub fn compile(mut self, parsed: Vec<Declaration>) -> Result<Program, CompilerError> {
        let locations = parsed
            .iter()
            .fold(0..0, |acc, decl| acc.start..decl.pos.end);

        for location in locations {
            self.scope_lookup.insert(location, self.scope_id);
        }

        self.instructions = self.get_instructions(&parsed)?;

        Ok(self)
    }
}
