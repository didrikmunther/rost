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
    DereferenceNonPointer(VariableType),
    MissingMainFunction,
    TooManyParametersInMainFunction,
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
    WrongArgumentType {
        parameter: VariableType,
        argument: VariableType,
        parameter_pos: Range<usize>,
    },
    WrongAssignmentType {
        got: VariableType,
        typ: VariableType,
        declaration_pos: Option<Range<usize>>,
    },
    
    #[allow(dead_code)]
    Todo {
        msg: String,
        file: &'static str,
        line: u32,
    },
}

#[macro_export]
macro_rules! compiler_todo {
    ($pos: expr, $msg: expr) => {{
        use super::error::{CompilerError, CompilerErrorKind};
        Err(CompilerError::new(
            $pos,
            CompilerErrorKind::Todo {
                msg: format!("{}", $msg),
                file: file!(),
                line: line!(),
            },
        ))
    }};
}

pub use compiler_todo;

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
            CompilerErrorKind::Todo { file, line, msg } => vec![(
                format!("Not yet implemented, {msg}. {file}:{line}"),
                self.pos.clone(),
            )],
            // todo: get_message should be a closure, accepting a document containing helper functions for getting lines.
            //  todo: perhaps a builder pattern to be able to show errors on multiple lines.
            CompilerErrorKind::MissingMainFunction => vec![("Missing main function".into(), 0..0)],
            CompilerErrorKind::TooManyParametersInMainFunction => vec![(
                "Too many parameters for main function, expected maximum of 2".into(),
                self.pos.clone(),
            )],
            CompilerErrorKind::DereferenceNonPointer(typ) => vec![(
                format!("Cannot dereference non-pointer value of type {typ}"),
                self.pos.clone(),
            )],
            CompilerErrorKind::RedeclaredVariable(identifier, pos) => vec![
                (
                    format!("Redeclared variable: {identifier}"),
                    self.pos.clone(),
                ),
                ("Already declared here".to_string(), pos.clone()),
            ],
            CompilerErrorKind::UndefinedVariable(identifier) => {
                vec![(
                    format!("Undefined variable: {identifier}"),
                    self.pos.clone(),
                )]
            }
            CompilerErrorKind::UndefinedFunction(identifier) => {
                vec![(
                    format!("Undefined function: {identifier}"),
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
                        format!("Incompatible types in binary expression: {got}"),
                        self.pos.clone(),
                    ),
                    (
                        format!("Operator {operator:?} is not defined for types."),
                        operator_pos.clone(),
                    ),
                    (format!("Other type is {expected}"), expected_pos.clone()),
                ]
            }
            CompilerErrorKind::WrongAssignmentType {
                got,
                typ,
                declaration_pos,
            } => {
                if let Some(pos) = declaration_pos {
                    vec![
                        (format!("Wrong type in assignment: {got}"), self.pos.clone()),
                        (format!("Variable declared with type {typ}"), pos.clone()),
                    ]
                } else {
                    vec![(format!("Wrong type in assignment: {got}"), self.pos.clone())]
                }
            }
            CompilerErrorKind::WrongArgumentType {
                argument,
                parameter,
                parameter_pos,
            } => {
                vec![
                    (
                        format!("Wrong type in argument: {argument}"),
                        self.pos.clone(),
                    ),
                    (
                        format!("Function takes parameter of type: {parameter}"),
                        parameter_pos.clone(),
                    ),
                ]
            }
            CompilerErrorKind::WrongType { got, expected } => {
                vec![(
                    format!("Wrong type: {got}, expected: {expected}"),
                    self.pos.clone(),
                )]
            }
        }
    }
}

impl From<CompilerError> for RostError {
    fn from(val: CompilerError) -> Self {
        RostError::new(
            "CompilerError".into(),
            val.get_messages()
                .iter()
                .map(RostErrorElement::from)
                .collect(),
        )
    }
}
