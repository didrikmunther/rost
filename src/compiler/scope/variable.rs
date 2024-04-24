use std::{
    fmt::Display,
    ops::{Deref, Range},
};

use crate::{lexer::Keyword, parser::types::Type};

/// Intermediate representation of a variable.
/// This type represents what the user wants to save,
/// while `StoredVariable` has a calculated `stack_pos`.
#[derive(Debug)]
pub struct Variable {
    pub pos: Range<usize>,
    pub typ: VariableType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructType {
    pub id: usize,
    pub typ: Type,
    pub size: usize, // size in bytes
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableType {
    Value(Keyword),
    Pointer(Box<VariableType>),
    Function(usize), // function id
    Struct(StructType),
}

impl Display for VariableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableType::Pointer(pointer) => {
                write!(f, "*")?;
                pointer.fmt(f)
            }
            VariableType::Value(keyword) => keyword.fmt(f),
            VariableType::Struct(struc) => struc.typ.fmt(f),
            _ => write!(f, "{self:?}"),
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
    Global(String),

    // The address has been pushed to the stack,
    // pop it and write to it.
    Address,
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
