use std::ops::Range;

use crate::error::RostError;

#[derive(Debug, PartialEq)]
pub enum NasmErrorKind {
    UnknownSystemCall(String),
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
            NasmErrorKind::UnknownSystemCall(ref s) => format!("Unknown system call: \"{}\"", s),
        }
    }
}

impl Into<RostError> for NasmError {
    fn into(self) -> RostError {
        RostError::new(self.get_message(), self.pos)
    }
}
