use crate::compiler::error::CompilerError;

use super::{
    builder::Builder,
    ir::{VariableId, VariableScope},
    Location, Program,
};
use std::{collections::HashMap, ops::Range};

#[derive(Debug, Default)]
pub struct Scope {
    pub variable_lookup: HashMap<String, VariableId>,
}

impl Scope {
    // Only keep global variables
    pub fn create_child(program: &Program) -> Self {
        Self {
            variable_lookup: program
                .get_scope()
                .variable_lookup
                .clone()
                .into_iter()
                .filter(|(_, variable_id)| {
                    program.variables.get(*variable_id).unwrap().scope == VariableScope::Global
                })
                .collect::<HashMap<_, _>>(),
        }
    }
}

pub type ScopeId = usize;

impl Program {
    pub fn get_scope_mut(&mut self) -> &mut Scope {
        self.scopes.get_mut(self.scope_id).unwrap()
    }

    pub fn get_scope(&self) -> &Scope {
        self.scopes.get(self.scope_id).unwrap()
    }

    /// Used when entering a new scope, makes sure that variables are properly scoped.
    /// Inner function should return the new builder and the range of the location of the scope.
    pub fn with_scope<F>(&mut self, inner: F) -> Result<Builder, CompilerError>
    where
        F: FnOnce(&mut Self) -> Result<(Builder, Range<Location>), CompilerError>,
    {
        let old_scope_id = self.scope_id;
        let scope_id = self.scopes.len();
        let scope = Scope::create_child(self);
        self.scopes.push(scope);
        self.scope_id = scope_id;

        let (result, locations) = inner(self)?;

        for location in locations {
            self.scope_lookup.insert(location, scope_id);
        }

        self.scope_id = old_scope_id;

        Ok(result)
    }
}
