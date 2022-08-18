use std::ops::Range;

use crate::error::RostError;

#[derive(Debug, PartialEq)]
pub enum NasmErrorKind {
    InvalidArgumentType(String),
    Unimplemented,
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
            NasmErrorKind::InvalidArgumentType(ref s) => {
                format!("Invalid argument type: \"{}\"", s)
            }
            NasmErrorKind::Unimplemented => "Unimplemented".into(),
        }
    }
}

impl Into<RostError> for NasmError {
    fn into(self) -> RostError {
        RostError::new("NasmError".into(), self.get_message(), self.pos)
    }
}
