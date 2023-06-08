use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    ops::Range,
};

use crate::parser::definition::FunctionDeclarationParameter;

use super::{
    builder::Builder,
    scope::variable::{VariableLocation, VariableType},
};

#[derive(Debug)]
pub struct Procedure {
    pub pos: Range<usize>,
    pub kind: ProcedureKind,
    pub comment: Option<String>,
}

impl Procedure {
    pub fn new(pos: Range<usize>, kind: ProcedureKind) -> Self {
        Self {
            pos,
            kind,
            comment: None,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)] // todo: remove
pub enum ProcedureKind {
    Comment(String),
    Allocate(usize), // Allocate a certain amount of variables on the stack
    Deref,
    Push(OperandValue),
    PushAddress(OperandValue),
    Assign(Assign), // Stack position of the variable to assign
    Arithmetic(Arithmetic),
    SystemCall(SystemCall),
    ProcedureCall(ProcedureCall),
    Return,
    If(Vec<If>),
    While(While),
}

impl Display for ProcedureKind {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ProcedureKind::If(ifs) => fmt.write_fmt(format_args!("If (n_cases: {})", ifs.len())),
            ProcedureKind::While(while_statement) => fmt.write_fmt(format_args!(
                "While (n_declarations: {})",
                while_statement.content.procedures.len()
            )),
            _ => fmt.write_fmt(format_args!("{self:?}")),
        }
    }
}

#[derive(Debug)]
pub struct Assign {
    // The start location of
    // the variable to assign to.
    pub location: VariableLocation,

    // The size of the object
    // being assigned.
    pub size: usize,
}

#[derive(Debug)]
pub struct While {
    pub condition: Box<Builder>,
    pub content: Box<Builder>,
}

#[derive(Debug)]
pub struct If {
    pub condition: Option<Box<Builder>>,
    pub content: Box<Builder>,
}

#[derive(Debug, Clone)]
pub enum Arithmetic {
    Add,
    Subtract,
    Multiply,
    Divide,
    LessThan,
    GreaterThan,
    Equality,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub identifier: String,
    pub parameters: Vec<String>,
    pub content: Box<Builder>,
}

#[derive(Debug)]
pub struct Function {
    pub return_type: Option<VariableType>,
    pub parameters: Vec<FunctionDeclarationParameter>,
    pub body: Builder,
}

#[derive(Debug)]
pub struct StructField {
    pub typ: VariableType,
    pub offset: usize,
    pub size: usize,
    pub pos: Range<usize>,
}

#[derive(Debug)]
pub struct Struct {
    pub fields: HashMap<String, StructField>,
    pub size: usize, // Size of struct in bytes
}

#[derive(Debug)]
pub struct SystemCall {
    pub identifier: String,
    pub nargs: usize,
}

#[derive(Debug)]
pub struct ProcedureCall {
    pub function_id: usize,
    pub nargs: usize,
    pub returns: bool, // If the called function returns a value
}

#[derive(Debug)]
pub struct Assignment {
    pub identifier: String,
}

#[derive(Debug)]
pub enum GlobalData {
    // Initialized .data literals
    String(String),

    // Uninitialized .bss data
    Reserved(usize),
}

#[derive(Debug)]
pub enum OperandValue {
    StackLocation(isize), // usize relative to stack
    DataLocation(String),
    DataPointerLocation(String),
    Int(i32),
}
