use std::ops::Range;

use crate::error::RostError;

#[derive(Debug, PartialEq)]
pub enum CompilerErrorKind {
    Unimplemented(String),
    UndefinedVariable(String),
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
