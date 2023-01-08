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

#[derive(Debug)]
pub enum VariableType {
    Value(Keyword),
    Function(usize), // function id
}

impl VariableType {
    pub fn to_keyword(&self) -> Keyword {
        match *self {
            Self::Value(keyword) => keyword,
            Self::Function(_) => Keyword::Function,
        }
    }
}

#[derive(Debug, Clone)]
pub enum VariableLocation {
    // The stack position will be negative for
    // arguments to functions, since
    // they reside below `rbp`.
    Stack(isize),

    // Name of global label
    Global(String)
}

#[derive(Debug)]
pub struct StoredVariable {
    pub variable: Variable,
    pub location: VariableLocation
}

impl Deref for StoredVariable {
    type Target = Variable;

    fn deref(&self) -> &Self::Target {
        &self.variable
    }
}
