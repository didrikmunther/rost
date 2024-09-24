use super::program::{typ::ExpressedType, Program};
use crate::{
    error::{RostError, RostErrorElement},
    lexer::Keyword,
};
use std::{
    fmt::{Display, Formatter},
    ops::Range,
};

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

#[derive(Debug, Clone)]
pub struct WrongType {
    pub content: String,
}

impl WrongType {
    pub fn from_expressed_type(_typ: ExpressedType, _program: &Program) -> Self {
        Self {
            content: _program
                .types
                .get(_typ.id)
                .unwrap()
                .format(_program)
                .to_string(),
        }
    }
}

impl Display for WrongType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

#[derive(Debug, Clone)]
pub struct WrongFunctionArguments {
    pub expected: String,
    pub got: String,
    pub declaration_pos: Range<usize>,
}

impl WrongFunctionArguments {
    pub fn from_expressed_type(
        expected: usize,
        got: usize,
        declaration_pos: Range<usize>,
        program: &Program,
    ) -> Self {
        Self {
            got: program.types.get(got).unwrap().format(program).to_string(),
            expected: program
                .types
                .get(expected)
                .unwrap()
                .clone()
                .format(program)
                .to_string(),
            declaration_pos,
        }
    }
}

#[derive(Debug, Clone)]
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
    WrongFunctionArguments(WrongFunctionArguments),
    RedeclaredVariable(String, Range<usize>),
    NotSupported(String),
    UndefinedType(WrongType),
    WrongAssignmentType {
        got: WrongType,
        typ: WrongType,
        declaration_pos: Option<Range<usize>>,
    },
    WrongBinaryExpressionTypes {
        got: WrongType,
        expected: WrongType,
        expected_pos: Range<usize>,
        operator: Keyword,
        operator_pos: Range<usize>,
    },

    #[allow(dead_code)]
    Todo {
        msg: String,
        file: &'static str,
        line: u32,
    },
}

#[derive(Debug, Clone)]
pub struct CompilerError {
    pub pos: Range<usize>,
    pub kind: Box<CompilerErrorKind>,
}

// Todo: Allow errors without positions,
// todo: such as no-main function.
impl CompilerError {
    pub fn new(pos: Range<usize>, kind: CompilerErrorKind) -> Self {
        Self {
            pos,
            kind: Box::new(kind),
        }
    }

    fn get_messages(&self) -> Vec<(String, Range<usize>)> {
        match &self.kind.as_ref() {
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
            CompilerErrorKind::WrongFunctionArguments(WrongFunctionArguments {
                expected,
                got,
                declaration_pos,
            }) => {
                vec![
                    (
                        format!("Wrong type in function call: {got}"),
                        self.pos.clone(),
                    ),
                    (
                        format!("Expected {expected} in declaration"),
                        declaration_pos.clone(),
                    ),
                ]
            }
            CompilerErrorKind::UndefinedType(WrongType { content }) => {
                vec![(format!("Undefined type: {content}"), self.pos.clone())]
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
