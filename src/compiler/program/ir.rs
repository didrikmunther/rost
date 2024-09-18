use std::ops::Range;

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

pub type VariableId = usize;
pub type GlobalDataId = usize;

#[derive(Debug)]
pub enum PrimitiveType {
    Int,
    Float,
    Char,
}

#[derive(Debug)]
pub enum PrimitiveValue {
    Int(i32),
    Float(f32),
    String(String),
}

#[derive(Debug, PartialEq)]
pub enum VariableScope {
    Local,
    Global,
}

#[derive(Debug)]
pub struct NormalVariable {
    pub typ: PrimitiveType,
}

#[derive(Debug, Default)]
pub struct Function {
    pub parameter_variable_ids: Vec<VariableId>,
    pub body: Builder,
}

#[derive(Debug)]
pub enum VariableKind {
    Normal(NormalVariable),
    DeclaredFunction(Function),
}

#[derive(Debug)]
pub struct Variable {
    pub identifier: String,
    pub scope: VariableScope,
    pub kind: VariableKind,
    pub declaration_pos: Range<usize>,
}

#[derive(Debug)]
pub enum ValueKind {
    Primitive(PrimitiveValue),
    Variable(VariableId),
    GlobalData(GlobalDataId),
}

#[derive(Debug)]
pub enum InstructionKind {
    Noop,
    Push(ValueKind),
    Pop,
    Assign(VariableId),
    IntAdd,
    IntMul,
    ProcedureCall(ProcedureCall),
    SystemCall(ProcedureCall),
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
pub struct ProcedureCall {
    pub identifier: String,
    pub nargs: usize,
}

#[derive(Debug)]
pub struct Assignment {
    pub identifier: String,
}
