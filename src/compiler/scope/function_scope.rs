use std::{collections::HashMap, rc::Rc};

use crate::parser::definition::ReturnType;

use super::{
    scope::Scope,
    variable::{StoredVariable, Variable, VariableLocation},
    ProgramScope,
};

#[derive(Debug)]
pub struct FunctionScope {
    parent: Option<Box<ProgramScope>>,
    pub scope: Scope,
    pub variables: HashMap<String, Rc<StoredVariable>>,
    pub return_type: ReturnType,

    // Keep track of assigned variables.
    // Will grow by +1 for each declared variable.
    stack_pos: usize,

    // Keep track of assigned parameters
    // The value will grow negatively,
    // since parameters reside below `rbp`.
    par_pos: isize,
}

impl FunctionScope {
    pub fn new(return_type: ReturnType) -> Self {
        Self {
            parent: None,
            scope: Scope::new(),
            variables: HashMap::new(),
            return_type,
            stack_pos: 1,

            // First two elements are
            // old rbp and return address,
            // therefore arguments start at -2.
            par_pos: -2,
        }
    }

    pub fn set_parent(&mut self, parent: Box<ProgramScope>) {
        self.parent = Some(parent);
    }

    pub fn take_parent(&mut self) -> ProgramScope {
        *self.parent.take().unwrap()
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
            location: VariableLocation::Stack(self.stack_pos as isize),
        };

        self.stack_pos += 1;

        stored
    }

    fn create_stored_parameter(&mut self, variable: Variable) -> StoredVariable {
        let stored = StoredVariable {
            variable,
            location: VariableLocation::Stack(self.par_pos),
        };

        self.par_pos -= 1;

        stored
    }

    /// Creates a stack allocated variable in the current function scope.
    /// Also adds it to the current scope variable lookup.
    /// Returns the location of the variable.
    pub fn create_variable(&mut self, identifier: String, variable: Variable) -> VariableLocation {
        let stored = Rc::new(self.create_stored_variable(variable));
        let location = stored.location.clone();

        self.variables.insert(
            self.scope.get_scoped_variable_name(&identifier),
            stored.clone(),
        );
        self.scope.insert_variable(identifier, stored);

        location
    }

    /// Creates a stack allocated parameter similarly to `create_variable`.
    pub fn create_parameter(&mut self, identifier: String, variable: Variable) -> VariableLocation {
        let stored = Rc::new(self.create_stored_parameter(variable));
        let location = stored.location.clone();

        self.variables.insert(
            self.scope.get_scoped_variable_name(&identifier),
            stored.clone(),
        );
        self.scope.insert_variable(identifier, stored);

        location
    }

    pub fn get_variable(&self, identifier: &String) -> Option<&StoredVariable> {
        self.scope.get_variable(identifier).or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| match parent.as_ref() {
                    ProgramScope::RootScope(root_scope) => root_scope.get_variable(identifier),
                    _ => None,
                })
        })
    }
}
