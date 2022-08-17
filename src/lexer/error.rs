use std::ops::Range;

use crate::error::RostError;

#[derive(Debug, PartialEq)]
pub enum LexerErrorKind {
    UnexpectedToken(char),
    UnknownEscapeSequence(char),
    UnterminatedQuote,
}

#[derive(Debug, PartialEq)]
pub struct LexerError {
    pub pos: Range<usize>,
    pub kind: LexerErrorKind,
}

impl LexerError {
    pub fn new(pos: Range<usize>, kind: LexerErrorKind) -> Self {
        Self { pos, kind }
    }

    fn get_message(&self) -> String {
        match self.kind {
            LexerErrorKind::UnexpectedToken(c) => format!("Unexpected token {}", c),
            LexerErrorKind::UnknownEscapeSequence(c) => format!("Unknown escape sequence '\\{}'", c),
            LexerErrorKind::UnterminatedQuote => "Unterminated quote".to_string(),
        }
    }
}

impl Into<RostError> for LexerError {
    fn into(self) -> RostError {
        RostError::new("LexerError".into(), self.get_message(), self.pos)
    }
}
