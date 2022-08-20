use std::ops::Range;

use crate::{
    error::{RostError, RostErrorElement},
    lexer::{Keyword, Token},
};

#[derive(Debug, PartialEq)]
pub enum ParserErrorKind {
    Unknown,
    UnexpectedToken(Token),
    Expected(&'static [Keyword]),
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

    fn get_messages(&self) -> Vec<(String, Range<usize>)> {
        match &self.kind {
            ParserErrorKind::UnterminatedParenthesis => {
                vec![("Unterminated parenthesis".to_string(), self.pos.clone())]
            }
            ParserErrorKind::UnexpectedEOF => {
                vec![("Unexpected EOF".to_string(), self.pos.clone())]
            }
            ParserErrorKind::UnexpectedToken(t) => {
                vec![(format!("Unexpected token: {:?}", t), self.pos.clone())]
            }
            ParserErrorKind::Expected(k) => vec![(format!("Expected: {:?}", k), self.pos.clone())],
            ParserErrorKind::Unknown => vec![("Unknown".to_string(), self.pos.clone())],
        }
    }
}

impl Into<RostError> for ParserError {
    fn into(self) -> RostError {
        RostError::new(
            "ParserError".into(),
            self.get_messages()
                .iter()
                .map(RostErrorElement::from)
                .collect(),
        )
    }
}
