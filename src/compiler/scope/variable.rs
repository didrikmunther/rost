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

#[derive(Debug)]
pub struct StoredVariable {
    pub variable: Variable,

    // The stack_pos will be negative for
    // arguments to functions, since
    // they reside below `rbp`.
    pub stack_pos: isize,
}

impl Deref for StoredVariable {
    type Target = Variable;

    fn deref(&self) -> &Self::Target {
        &self.variable
    }
}
