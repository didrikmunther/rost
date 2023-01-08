use std::{collections::HashMap, rc::Rc};

use super::{
    scope::Scope,
    variable::{StoredVariable, Variable, VariableLocation},
};

#[derive(Debug)]
pub struct RootScope {
    pub scope: Scope,
    pub variables: HashMap<String, Rc<StoredVariable>>,
}

impl RootScope {
    pub fn new() -> Self {
        Self {
            scope: Scope::new(),
            variables: HashMap::new(),
        }
    }

    /// Opens a new scope
    pub fn create_scope(&mut self) {
        let mut scope = Scope::new();
        std::mem::swap(&mut scope, &mut self.scope);
        self.scope.set_parent(Box::new(scope));
    }

    /// Close the current scope
    pub fn close_scope(&mut self) {
        self.scope = self.scope.take_parent();
    }

    /// Creates a global allocated variable in the root scope.
    /// Returns the label name of the variable.
    pub fn create_variable(&mut self, identifier: String, variable: Variable) -> VariableLocation {
        let location = VariableLocation::Global(format!("_global_{}", identifier));
        let stored = Rc::new(StoredVariable {
            variable,
            location: location.clone(),
        });

        self.variables.insert(
            self.scope.get_scoped_variable_name(&identifier),
            stored.clone(),
        );

        self.scope.insert_variable(identifier, stored);

        location
    }

    pub fn get_variable(&self, identifier: &String) -> Option<&StoredVariable> {
        self.scope.get_variable(identifier)
    }
}
