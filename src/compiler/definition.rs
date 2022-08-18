use std::ops::Range;

#[derive(Debug)]
pub struct Procedure {
    pub pos: Range<usize>,
    pub kind: ProcedureKind,
}

#[derive(Debug)]
pub enum ProcedureKind {
    Assignment(Assignment),
    SystemCall(SystemCall),
}

#[derive(Debug)]
pub struct SystemCall {
    pub identifier: String,
    pub args: Vec<OperandValue>,
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
