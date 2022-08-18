use std::ops::Range;

#[derive(Debug)]
pub struct Procedure {
    pub pos: Range<usize>,
    pub kind: ProcedureKind,
}

impl Procedure {
    pub fn new(pos: Range<usize>, kind: ProcedureKind) -> Self {
        Self { pos, kind }
    }
}

#[derive(Debug)]
pub enum ProcedureKind {
    Push(OperandValue),
    Arithmetic(Arithmetic),
    SystemCall(SystemCall),
}

#[derive(Debug)]
pub enum Arithmetic {
    Add,
    Subtract,
    Multiply,
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
