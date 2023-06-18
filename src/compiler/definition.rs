use std::{
    collections::BTreeMap,
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
pub enum ProcedureKind {
    Comment(String),
    Allocate(usize), // Allocate a certain amount of variables on the stack
    Deref,
    Push(OperandValue),
    PushAddress(OperandValue),
    Assign(Assign), // Stack position of the variable to assign
    Arithmetic(Arithmetic, RegisterSize),
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
#[allow(dead_code)]
pub enum RegisterSize {
    B64,
    B32,
    B16,
    B8
}

impl RegisterSize {
    pub fn get_register_name(&self, register: &str) -> String {
        use RegisterSize::*;
        match self {
            B64 => format!("R{register}X"),
            B32 => format!("E{register}X"),
            B16 => format!("{register}X"),
            B8 => format!("{register}L"),
        }
    }

    pub fn get_register(type_size: usize) -> Self {
        use RegisterSize::*;
        match type_size {
            1 => B8,
            2 => B16,
            4 => B32,
            8 => B64,
            _ => todo!("Cannot create register from size {type_size}")
        }
    }

    fn get_ordering(&self) -> usize {
        use RegisterSize::*;
        match self {
            B64 => 3,
            B32 => 2,
            B16 => 1,
            B8 => 0,
        }
    }

    pub fn get_smallest(self, register: RegisterSize) -> RegisterSize {
        if self.get_ordering() < register.get_ordering() {
            self
        } else {
            register
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
    pub identifier_pos: Range<usize>,
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
    pub fields: BTreeMap<String, StructField>,
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
