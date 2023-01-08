use std::ops::{Deref, Range};

use crate::lexer::Keyword;

/// Intermediate representation of a variable.
/// This type represents what the user wants to save,
/// while `StoredVariable` has a calculated `stack_pos`.
#[derive(Debug)]
pub struct Variable {
    pub pos: Range<usize>,
    pub typ: VariableType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableType {
    Value(Keyword),
    Pointer(Box<VariableType>),
    Function(usize), // function id
}

#[derive(Debug, Clone)]
pub enum VariableLocation {
    // The stack position will be negative for
    // arguments to functions, since
    // they reside below `rbp`.
    Stack(isize),

    // Name of global label
    Global(String),
}

#[derive(Debug)]
pub struct StoredVariable {
    pub variable: Variable,
    pub location: VariableLocation,
}

impl Deref for StoredVariable {
    type Target = Variable;

    fn deref(&self) -> &Self::Target {
        &self.variable
    }
}
