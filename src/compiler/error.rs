use std::ops::Range;

use crate::error::RostError;

#[derive(Debug, PartialEq)]
pub enum CompilerErrorKind {
    Unimplemented(String),
    UndefinedVariable(String),
    RedeclaredVariable(String, Range<usize>),
}

#[derive(Debug, PartialEq)]
pub struct CompilerError {
    pub pos: Range<usize>,
    pub kind: CompilerErrorKind,
}

impl CompilerError {
    pub fn new(pos: Range<usize>, kind: CompilerErrorKind) -> Self {
        Self { pos, kind }
    }

    fn get_message(&self) -> String {
        match &self.kind {
            CompilerErrorKind::Unimplemented(s) => format!("Unimplemented: {}", s),
             // todo: get_message should be a closure, accepting a document containing helper functions for getting lines.
            //  todo: perhaps a builder pattern to be able to show errors on multiple lines.
            CompilerErrorKind::RedeclaredVariable(identifier, pos) => format!("Redeclared variable: {}. Already declared at {:?}", identifier, pos),
            CompilerErrorKind::UndefinedVariable(identifier) => {
                format!("Undefined variable: {}", identifier)
            }
        }
    }
}

impl Into<RostError> for CompilerError {
    fn into(self) -> RostError {
        RostError::new("CompilerError".into(), self.get_message(), self.pos)
    }
}
