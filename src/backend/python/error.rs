use std::ops::Range;

use crate::error::{RostError, RostErrorElement};

#[derive(Debug, PartialEq)]
pub enum PythonErrorKind {
    MainFunctionNotFound,
}

impl PythonErrorKind {
    pub fn at_pos(self, pos: &Range<usize>) -> PythonError {
        PythonError::new(pos.clone(), self)
    }
}

impl<T> From<PythonError> for Result<T, PythonError> {
    fn from(val: PythonError) -> Self {
        Err(val)
    }
}

#[derive(Debug, PartialEq)]
pub struct PythonError {
    pub pos: Range<usize>,
    pub kind: PythonErrorKind,
}

impl PythonError {
    pub fn new(pos: Range<usize>, kind: PythonErrorKind) -> Self {
        Self { pos, kind }
    }

    fn get_messages(&self) -> Vec<(String, Range<usize>)> {
        match self.kind {
            PythonErrorKind::MainFunctionNotFound => {
                vec![("Main function not found".into(), self.pos.clone())]
            } // PythonErrorKind::TooManyArguments(a) => vec![(
              //     format!("Too many arguments ({a}) to function (no more than 6 supported)"),
              //     self.pos.clone(),
              // )],
        }
    }
}

impl From<PythonError> for RostError {
    fn from(val: PythonError) -> Self {
        RostError::new(
            "PythonError".into(),
            val.get_messages()
                .iter()
                .map(RostErrorElement::from)
                .collect(),
        )
    }
}
