use std::ops::Range;

use crate::{
    error::{RostError, RostErrorElement},
    lexer::Keyword,
    parser::definition::Type,
};

#[derive(Debug, PartialEq)]
pub enum CompilerErrorKind {
    UndefinedVariable(String),
    RedeclaredVariable(String, Range<usize>),
    WrongBinaryExpressionTypes {
        got: Type,
        expected: Type,
        expected_pos: Range<usize>,
    },
    WrongType {
        got: Type,
        expected: Type,
    },
    WrongAssignmentType {
        got: Type,
        typ: Keyword,
        declaration_pos: Range<usize>,
    },
}

#[derive(Debug, PartialEq)]
pub struct CompilerError {
    pub pos: Range<usize>,
    pub kind: CompilerErrorKind,
}

impl CompilerError {
    pub fn new(pos: Range<usize>, kind: CompilerErrorKind) -> Self {
        Self { pos, kind }
    }

    fn get_messages(&self) -> Vec<(String, Range<usize>)> {
        match &self.kind {
            // todo: get_message should be a closure, accepting a document containing helper functions for getting lines.
            //  todo: perhaps a builder pattern to be able to show errors on multiple lines.
            CompilerErrorKind::RedeclaredVariable(identifier, pos) => vec![
                (
                    format!("Redeclared variable: {}", identifier),
                    self.pos.clone(),
                ),
                ("Already declared here".to_string(), pos.clone()),
            ],
            CompilerErrorKind::UndefinedVariable(identifier) => {
                vec![(
                    format!("Undefined variable: {}", identifier),
                    self.pos.clone(),
                )]
            }
            CompilerErrorKind::WrongBinaryExpressionTypes {
                got,
                expected,
                expected_pos,
            } => {
                vec![
                    (
                        format!("Incompatible types in binary expression: {:?}", got),
                        self.pos.clone(),
                    ),
                    (
                        format!("Other type is {:?}", expected),
                        expected_pos.clone(),
                    ),
                ]
            }
            CompilerErrorKind::WrongAssignmentType {
                got,
                typ,
                declaration_pos,
            } => {
                vec![
                    (
                        format!("Wrong type in assignment: {:?}", got),
                        self.pos.clone(),
                    ),
                    (
                        format!("Variable declared with type {:?}", typ),
                        declaration_pos.clone(),
                    ),
                ]
            }
            CompilerErrorKind::WrongType { got, expected } => {
                vec![(
                    format!("Wrong type: {:?}, expected: {:?}", got, expected),
                    self.pos.clone(),
                )]
            }
        }
    }
}

impl Into<RostError> for CompilerError {
    fn into(self) -> RostError {
        RostError::new(
            "CompilerError".into(),
            self.get_messages()
                .iter()
                .map(RostErrorElement::from)
                .collect(),
        )
    }
}
