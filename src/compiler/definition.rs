use std::ops::Range;

use super::builder::Builder;

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
    Push(OperandValue),
    Reassign(usize), // stack location
    Arithmetic(Arithmetic),
    SystemCall(SystemCall),
    If(Vec<If>),
    While(While),
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

#[derive(Debug)]
pub enum Arithmetic {
    Add,
    Subtract,
    Multiply,
    LessThan,
    GreaterThan,
    Equality,
}

#[derive(Debug)]
pub struct SystemCall {
    pub identifier: String,
    pub nargs: usize,
}

#[derive(Debug)]
pub struct Assignment {
    pub identifier: String,
}

#[derive(Debug)]
pub struct GlobalData {
    pub content: String, // todo: can be all sorts of bytes
}

#[derive(Debug)]
pub enum OperandValue {
    StackLocation(usize), // usize relative to stack
    ByteLocation(usize),
    Int(i32),
}
