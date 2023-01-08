use std::ops::Range;

use crate::{
    error::{RostError, RostErrorElement},
    lexer::Keyword,
};

use super::scope::variable::VariableType;

#[derive(Debug, PartialEq)]
pub enum CompilerErrorKind {
    UndefinedVariable(String),
    UndefinedFunction(String),
    RedeclaredVariable(String, Range<usize>),
    MissingMainFunction,
    WrongBinaryExpressionTypes {
        got: VariableType,
        expected: VariableType,
        expected_pos: Range<usize>,
        operator: Keyword,
        operator_pos: Range<usize>,
    },
    WrongType {
        got: VariableType,
        expected: VariableType,
    },
    WrongAssignmentType {
        got: VariableType,
        typ: VariableType,
        declaration_pos: Range<usize>,
    },
}

#[derive(Debug, PartialEq)]
pub struct CompilerError {
    pub pos: Range<usize>,
    pub kind: CompilerErrorKind,
}

// Todo: Allow errors without positions,
// todo: such as no-main function.
impl CompilerError {
    pub fn new(pos: Range<usize>, kind: CompilerErrorKind) -> Self {
        Self { pos, kind }
    }

    fn get_messages(&self) -> Vec<(String, Range<usize>)> {
        match &self.kind {
            // todo: get_message should be a closure, accepting a document containing helper functions for getting lines.
            //  todo: perhaps a builder pattern to be able to show errors on multiple lines.
            CompilerErrorKind::MissingMainFunction => vec![("Missing main function".into(), 0..0)],
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
            CompilerErrorKind::UndefinedFunction(identifier) => {
                vec![(
                    format!("Undefined function: {}", identifier),
                    self.pos.clone(),
                )]
            }
            CompilerErrorKind::WrongBinaryExpressionTypes {
                got,
                expected,
                expected_pos,
                operator,
                operator_pos,
            } => {
                vec![
                    (
                        format!("Incompatible types in binary expression: {}", got),
                        self.pos.clone(),
                    ),
                    (
                        format!("Operator {:?} is not defined for types.", operator),
                        operator_pos.clone(),
                    ),
                    (format!("Other type is {}", expected), expected_pos.clone()),
                ]
            }
            CompilerErrorKind::WrongAssignmentType {
                got,
                typ,
                declaration_pos,
            } => {
                vec![
                    (
                        format!("Wrong type in assignment: {}", got),
                        self.pos.clone(),
                    ),
                    (
                        format!("Variable declared with type {}", typ),
                        declaration_pos.clone(),
                    ),
                ]
            }
            CompilerErrorKind::WrongType { got, expected } => {
                vec![(
                    format!("Wrong type: {}, expected: {}", got, expected),
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
