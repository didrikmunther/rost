use std::{
    fmt::{Display, Formatter},
    ops::Range,
};

use crate::parser::definition::FunctionDeclarationParameter;

use super::builder::Builder;

// An instruction is an atomic intermediate representation of an instruction

#[derive(Debug)]
pub struct Instruction {
    pub pos: Range<usize>,
    pub kind: InstructionKind,
    pub comment: Option<String>,
}

impl Instruction {
    pub fn new(pos: Range<usize>, kind: InstructionKind) -> Self {
        Self {
            pos,
            kind,
            comment: None,
        }
    }

    pub fn with_comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }
}

#[derive(Debug)]
pub enum PrimitiveType {
    Int,
    Float,
    Char,
}

pub type VariableId = usize;

#[derive(Debug)]
pub struct Variable {
    pub name: Option<String>,
    pub typ: PrimitiveType,
}

#[derive(Debug)]
pub enum ValueKind {
    Int(i32),
    Variable(VariableId),
    // GlobalVariable(VariableId),
}

#[derive(Debug)]
pub enum InstructionKind {
    Noop,
    Assign(VariableId, ValueKind),
    BinaryOperation {
        operator: Arithmetic,
        left: VariableId,
        right: ValueKind,
    },
    // Deref,
    // Push(OperandValue),
    // PushAddress(OperandValue),
    // Assign(Assign), // Stack position of the variable to assign
    // Arithmetic(Arithmetic),
    // SystemCall(SystemCall),
    // ProcedureCall(ProcedureCall),
    // Return,
    // If(Vec<If>),
    // While(While),
}

impl Display for InstructionKind {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            // InstructionKind::If(ifs) => fmt.write_fmt(format_args!("If (n_cases: {})", ifs.len())),
            // InstructionKind::While(while_statement) => fmt.write_fmt(format_args!(
            //     "While (n_declarations: {})",
            //     while_statement.content.instructions.len()
            // )),
            _ => fmt.write_fmt(format_args!("{self:?}")),
        }
    }
}

// #[derive(Debug)]
// pub struct Assign {
//     // The start location of
//     // the variable to assign to.
//     pub location: VariableLocation,

//     // The size of the object
//     // being assigned.
//     pub size: usize,
// }

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
    // pub identifier_pos: Range<usize>,
    // pub return_type: Option<VariableType>,
    // pub parameters: Vec<FunctionDeclarationParameter>,
    pub body: Builder,
}

// #[derive(Debug)]
// pub struct StructField {
//     pub typ: VariableType,
//     pub offset: usize,
//     pub size: usize,
//     pub pos: Range<usize>,
// }

// #[derive(Debug)]
// pub struct Struct {
//     pub fields: BTreeMap<String, StructField>,
//     pub size: usize, // Size of struct in bytes
// }

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

// #[derive(Debug)]
// pub enum GlobalData {
//     // Initialized .data literals
//     String(String),

//     // Uninitialized .bss data
//     Reserved(usize),
// }

// #[derive(Debug)]
// pub enum OperandValue {
//     StackLocation(isize), // usize relative to stack
//     DataLocation(String),
//     DataPointerLocation(String),
//     Int(i32),
// }
