use std::ops::Range;

use crate::error::{RostError, RostErrorElement};

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

    fn get_messages(&self) -> Vec<(String, Range<usize>)> {
        match self.kind {
            NasmErrorKind::TooManyArguments(a) => vec![(
                format!("Too many arguments ({a}) to function (no more than 6 supported)"),
                self.pos.clone(),
            )],
        }
    }
}

impl From<NasmError> for RostError {
    fn from(val: NasmError) -> Self {
        RostError::new(
            "NasmError".into(),
            val.get_messages()
                .iter()
                .map(RostErrorElement::from)
                .collect(),
        )
    }
}
