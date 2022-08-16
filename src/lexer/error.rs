use std::ops::Range;

use crate::error::RostError;

#[derive(Debug, PartialEq)]
pub enum LexerErrorKind {
    UnexpectedToken(char),
    UnterminatedQuote,
}

#[derive(Debug, PartialEq)]
pub struct LexerError {
    pub pos: Range<usize>,
    pub kind: LexerErrorKind,
}

impl LexerError {
    fn get_message(&self) -> String {
        match self.kind {
            LexerErrorKind::UnexpectedToken(c) => format!("Unexpected token {}", c),
            LexerErrorKind::UnterminatedQuote => "Unterminated quote".to_string(),
        }
    }
}

impl Into<RostError> for LexerError {
    fn into(self) -> RostError {
        RostError::new(self.get_message(), self.pos)
    }
}
