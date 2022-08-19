use std::ops::Range;

use crate::{
    error::RostError,
    lexer::{Keyword, Token},
};

#[derive(Debug, PartialEq)]
pub enum ParserErrorKind {
    Unknown,
    UnexpectedToken(Token),
    Expected(&'static [Keyword]),
    CannotInferType,
    WrongType(Keyword),
    UnexpectedEOF,
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
        match &self.kind {
            ParserErrorKind::UnterminatedParenthesis => "Unterminated parenthesis".to_string(),
            ParserErrorKind::UnexpectedEOF => "Unexpected EOF".to_string(),
            ParserErrorKind::UnexpectedToken(t) => format!("Unexpected token: {:?}", t),
            ParserErrorKind::Expected(k) => format!("Expected: {:?}", k),
            ParserErrorKind::WrongType(k) => format!("Wrong type: {:?}", k),
            ParserErrorKind::CannotInferType => "Cannot infer assignment type".to_string(),
            ParserErrorKind::Unknown => "Unknown".to_string(),
        }
    }
}

impl Into<RostError> for ParserError {
    fn into(self) -> RostError {
        RostError::new("ParserError".into(), self.get_message(), self.pos)
    }
}
