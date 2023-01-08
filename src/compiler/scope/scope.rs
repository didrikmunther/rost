use std::{collections::HashMap, rc::Rc};

use super::variable::StoredVariable;

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
        format!("{}__{}", self.name, name)
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
