use crate::{lexer::Keyword, parser::definition::ReturnType};

use std::{collections::HashMap, ops::Range, rc::Rc};

/// Intermediate representation of a variable.
/// This type represents what the user wants to save,
/// while `StoredVariable` has a calculated `stack_pos`.
#[derive(Debug)]
pub struct Variable {
    pub pos: Range<usize>,
    pub typ: Keyword,
}

#[derive(Debug)]
pub struct StoredVariable {
    pub pos: Range<usize>,
    pub typ: Keyword,
    pub stack_pos: usize,
}

#[derive(Debug)]
pub struct FunctionScope {
    parent: Option<Box<FunctionScope>>,
    pub scope: Scope,
    pub variables: HashMap<String, Rc<StoredVariable>>,
    pub return_type: ReturnType,

    // Keep track of assigned variables
    stack_pos: usize,
}

impl FunctionScope {
    pub fn new(return_type: ReturnType) -> Self {
        Self {
            parent: None,
            scope: Scope::new(),
            variables: HashMap::new(),
            return_type,
            stack_pos: 0,
        }
    }

    pub fn set_parent(&mut self, parent: Box<FunctionScope>) {
        self.parent = Some(parent);
    }

    pub fn take_parent(&mut self) -> FunctionScope {
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
        let variable = StoredVariable {
            pos: variable.pos,
            typ: variable.typ,
            stack_pos: self.stack_pos,
        };

        self.stack_pos += 1;

        variable
    }

    /// Creates a stack allocated variable in the current function scope.
    /// Also adds it to the current scope variable lookup.
    /// Returns the stack position of the variable.
    pub fn create_variable(&mut self, identifier: String, variable: Variable) -> usize {
        let stored = Rc::new(self.create_stored_variable(variable));
        let stack_pos = stored.stack_pos;

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

#[derive(Debug)]
pub struct Scope {
    parent: Option<Box<Scope>>,
    pub variables: HashMap<String, Rc<StoredVariable>>,
    pub name: String,      // Prefix for variables
    n_child_scopes: usize, // Amount of child scopes
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            parent: None,
            variables: HashMap::new(),
            name: String::new(),
            n_child_scopes: 0,
        }
    }

    fn get_child_scope_name(&self) -> String {
        format!("{}_{}", self.name, self.n_child_scopes)
    }

    pub fn get_scoped_variable_name(&self, name: &str) -> String {
        format!("{}[{}]", self.name, name)
    }

    pub fn set_parent(&mut self, mut parent: Box<Scope>) {
        parent.as_mut().n_child_scopes += 1;
        self.name = parent.get_child_scope_name();
        self.parent = Some(parent);
    }

    pub fn take_parent(&mut self) -> Scope {
        *self.parent.take().unwrap()
    }

    pub fn get_variable(&self, identifier: &String) -> Option<&StoredVariable> {
        if let Some(var) = self.variables.get(identifier) {
            Some(var)
        } else if let Some(ref parent) = self.parent {
            parent.get_variable(identifier)
        } else {
            None
        }
    }

    /// Returns the identifier and variable if it was not set
    fn insert_variable_inner(
        &mut self,
        identifier: String,
        variable: Rc<StoredVariable>,
    ) -> Option<(String, Rc<StoredVariable>)> {
        if let Some(_) = self.variables.get(&identifier) {
            self.variables.insert(identifier, variable);
            None
        } else if let Some(ref mut parent) = self.parent {
            parent.insert_variable_inner(identifier, variable);
            None
        } else {
            Some((identifier, variable))
        }
    }

    pub fn insert_variable<'b>(&mut self, identifier: String, variable: Rc<StoredVariable>) {
        if let Some(_) = self.variables.get(&identifier) {
            self.variables.insert(identifier, variable);
        } else if let Some(ref mut parent) = self.parent {
            if let Some((identifier, variable)) = parent.insert_variable_inner(identifier, variable)
            {
                self.variables.insert(identifier, variable);
            }
        } else {
            self.variables.insert(identifier, variable);
        }
    }
}
