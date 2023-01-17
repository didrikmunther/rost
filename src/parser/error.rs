use std::ops::Range;

use crate::{
    error::{RostError, RostErrorElement},
    lexer::{Keyword, Token},
};

#[macro_export]
macro_rules! parser_todo {
    ($pos: expr, $msg: expr) => {{
        use super::error::{ParserError, ParserErrorKind};
        Err(ParserError::new(
            $pos,
            ParserErrorKind::Todo {
                msg: format!("{}", $msg),
                file: file!(),
                line: line!(),
            },
        ))
    }};
}

pub use parser_todo;

#[derive(Debug, PartialEq)]
pub enum ParserErrorKind {
    Unknown,
    Todo {
        msg: String,
        file: &'static str,
        line: u32,
    },
    UnexpectedToken(Token),
    Expected(&'static [Keyword]),
    ExpectedSemicolon,
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
            ParserErrorKind::Todo { file, line, msg } => {
                vec![(
                    format!("Not yet implemented, {}. {}:{}", msg, file, line),
                    self.pos.clone(),
                )]
            }
            ParserErrorKind::UnterminatedParenthesis => {
                vec![("Unterminated parenthesis".to_string(), self.pos.clone())]
            }
            ParserErrorKind::ExpectedSemicolon => {
                vec![("Expected terminating semicolon for statement".to_string(), self.pos.clone())]
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
