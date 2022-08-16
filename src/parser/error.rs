use std::ops::Range;

use crate::error::RostError;

#[derive(Debug, PartialEq)]
pub enum ParserErrorKind {
    Unknown,
    UnterminatedParenthesis,
}

#[derive(Debug, PartialEq)]
pub struct ParserError {
    pub pos: Range<usize>,
    pub kind: ParserErrorKind,
}

impl ParserError {
    pub fn new(pos: Range<usize>, kind: ParserErrorKind) -> Self {
        Self { pos, kind }
    }

    fn get_message(&self) -> String {
        match self.kind {
            ParserErrorKind::UnterminatedParenthesis => "Unterminated parenthesis".to_string(),
            ParserErrorKind::Unknown => "Unknown".to_string(),
        }
    }
}

impl Into<RostError> for ParserError {
    fn into(self) -> RostError {
        RostError::new(self.get_message(), self.pos)
    }
}
