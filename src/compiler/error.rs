use std::ops::Range;

use crate::error::RostError;

#[derive(Debug, PartialEq)]
pub enum CompilerErrorKind {
    Unimplemented,
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
        match self.kind {
            CompilerErrorKind::Unimplemented => "Unimplemented".to_string(),
        }
    }
}

impl Into<RostError> for CompilerError {
    fn into(self) -> RostError {
        RostError::new("CompilerError".into(), self.get_message(), self.pos)
    }
}
