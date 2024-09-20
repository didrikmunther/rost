use std::ops::Range;

use crate::error::{RostError, RostErrorElement};

// use super::scope::variable::VariableType;

#[derive(Debug, PartialEq, Clone)]
pub enum CompilerErrorKind {
    UndefinedVariable(String),
    UndefinedFunction(String),
    NotAFunction(String),
    NotEnoughFunctionArguments {
        missing: Vec<String>,
        got: usize,
    },
    TooManyFunctionArguments {
        expected: usize,
        got: usize,
    },
    RedeclaredVariable(String, Range<usize>),
    NotSupported(String),

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

#[allow(unused_imports)]
pub use compiler_todo;

#[derive(Debug, PartialEq, Clone)]
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
            CompilerErrorKind::NotAFunction(identifier) => {
                vec![(format!("Not a function: {identifier}"), self.pos.clone())]
            }
            CompilerErrorKind::NotEnoughFunctionArguments { missing, got } => {
                vec![(
                    format!(
                        "Not enough function arguments, only got {got}, missing: {missing}",
                        missing = missing.join(", "),
                    ),
                    self.pos.clone(),
                )]
            }
            CompilerErrorKind::TooManyFunctionArguments { expected, got } => {
                vec![(
                    format!("Too many function arguments, expected {expected}, got {got}",),
                    self.pos.clone(),
                )]
            }
            CompilerErrorKind::NotSupported(v) => {
                vec![(format!("Not supported: {v}"), self.pos.clone())]
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
