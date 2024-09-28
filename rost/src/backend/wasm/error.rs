use std::ops::Range;

use crate::error::{RostError, RostErrorElement};

#[derive(Debug, PartialEq)]
pub enum WasmErrorKind {
    // TooManyArguments(usize),
}

#[derive(Debug, PartialEq)]
pub struct WasmError {
    pub pos: Range<usize>,
    pub kind: WasmErrorKind,
}

impl WasmError {
    pub fn new(pos: Range<usize>, kind: WasmErrorKind) -> Self {
        Self { pos, kind }
    }

    fn get_messages(&self) -> Vec<(String, Range<usize>)> {
        match self.kind {
            // PythonErrorKind::TooManyArguments(a) => vec![(
            //     format!("Too many arguments ({a}) to function (no more than 6 supported)"),
            //     self.pos.clone(),
            // )],
        }
    }
}

impl From<WasmError> for RostError {
    fn from(val: WasmError) -> Self {
        RostError::new(
            "WasmError".into(),
            val.get_messages()
                .iter()
                .map(RostErrorElement::from)
                .collect(),
        )
    }
}
