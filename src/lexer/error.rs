use std::ops::Range;

use crate::error::{RostError, RostErrorElement};

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

    fn get_messages(&self) -> Vec<(String, Range<usize>)> {
        match self.kind {
            LexerErrorKind::UnexpectedToken(c) => {
                vec![(format!("Unexpected token {}", c), self.pos.clone())]
            }
            LexerErrorKind::UnknownEscapeSequence(c) => {
                vec![(
                    format!("Unknown escape sequence '\\{}'", c),
                    self.pos.clone(),
                )]
            }
            LexerErrorKind::UnterminatedQuote => {
                vec![("Unterminated quote".to_string(), self.pos.clone())]
            }
        }
    }
}

impl Into<RostError> for LexerError {
    fn into(self) -> RostError {
        RostError::new(
            "LexerError".into(),
            self.get_messages()
                .iter()
                .map(RostErrorElement::from)
                .collect(),
        )
    }
}
