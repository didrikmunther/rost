use std::ops::Range;

use crate::error::RostError;

#[derive(Debug, PartialEq)]
pub enum NasmErrorKind {
    TooManyArguments(usize),
}

#[derive(Debug, PartialEq)]
pub struct NasmError {
    pub pos: Range<usize>,
    pub kind: NasmErrorKind,
}

impl NasmError {
    pub fn new(pos: Range<usize>, kind: NasmErrorKind) -> Self {
        Self { pos, kind }
    }

    fn get_message(&self) -> String {
        match self.kind {
            NasmErrorKind::TooManyArguments(a) => format!(
                "Too many arguments ({}) to function (no more than 6 supported)",
                a
            ),
        }
    }
}

impl Into<RostError> for NasmError {
    fn into(self) -> RostError {
        RostError::new("NasmError".into(), self.get_message(), self.pos)
    }
}
