use crate::lexer::Keyword;

use std::{collections::HashMap, ops::Range};

use super::error::CompilerError;

#[derive(Debug)]
pub struct Variable {
    pub pos: Range<usize>,
    pub typ: Keyword,
    pub stack_pos: usize,
}

#[derive(Debug)]
pub struct Scope {
    pub parent: Option<Box<Scope>>,
    pub variables: HashMap<String, Variable>,
}

impl<'a> Scope {
    pub fn new() -> Scope {
        Scope {
            parent: None,
            variables: HashMap::new(),
        }
    }

    pub fn take_parent(&mut self) -> Scope {
        *self.parent.take().unwrap()
    }

    pub fn get_variable(&'a self, identifier: &String) -> Option<&'a Variable> {
        if let Some(var) = self.variables.get(identifier) {
            Some(var)
        } else if let Some(ref parent) = self.parent {
            parent.get_variable(identifier)
        } else {
            None
        }
    }

    /// Returns true if a variable was found and set
    pub fn insert_variable_inner(&'a mut self, identifier: String, variable: Variable) -> bool {
        if let Some(_) = self.variables.get(&identifier) {
            self.variables.insert(identifier, variable);
            true
        } else if let Some(ref mut parent) = self.parent {
            parent.insert_variable_inner(identifier, variable)
        } else {
            false
        }
    }

    pub fn insert_variable(&'a mut self, identifier: String, variable: Variable) {
        if let Some(_) = self.variables.get(&identifier) {
            self.variables.insert(identifier, variable);
        } else if let Some(ref mut parent) = self.parent {
            if !parent.insert_variable_inner(identifier, variable) {}
        } else {
            self.variables.insert(identifier, variable);
        }
    }
}
