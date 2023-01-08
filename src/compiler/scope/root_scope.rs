use std::{collections::HashMap, rc::Rc};

use super::{
    scope::Scope,
    variable::{StoredVariable, Variable},
};

#[derive(Debug)]
pub struct RootScope {
    pub scope: Scope,
    pub variables: HashMap<String, Rc<StoredVariable>>,

    // Keep track of assigned variables.
    // Will grow by +1 for each declared variable.
    stack_pos: usize,
}

impl RootScope {
    pub fn new() -> Self {
        Self {
            scope: Scope::new(),
            variables: HashMap::new(),
            stack_pos: 1,
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

    fn create_stored_variable(&mut self, variable: Variable) -> StoredVariable {
        let stored = StoredVariable {
            variable,
            stack_pos: self.stack_pos as isize,
        };

        self.stack_pos += 1;

        stored
    }

    /// Creates a stack allocated variable in the current function scope.
    /// Also adds it to the current scope variable lookup.
    /// Returns the stack position of the variable.
    pub fn create_variable(&mut self, identifier: String, variable: Variable) -> usize {
        let stored = Rc::new(self.create_stored_variable(variable));
        let stack_pos = stored.stack_pos.try_into().unwrap();

        self.variables.insert(
            self.scope.get_scoped_variable_name(&identifier),
            stored.clone(),
        );
        self.scope.insert_variable(identifier, stored);

        stack_pos
    }

    pub fn get_variable(&self, identifier: &String) -> Option<&StoredVariable> {
        self.scope.get_variable(identifier)
    }
}
