use std::ops::Range;

#[derive(Debug)]
pub struct Procedure {
    pub pos: Range<usize>,
    pub kind: ProcedureKind,
}

#[derive(Debug)]
pub enum ProcedureKind {
    SystemCall(SystemCall),
}

#[derive(Debug)]
pub struct SystemCall {
    pub identifier: String,
    pub args: Vec<RegisterValue>,
}

#[derive(Debug)]
pub struct GlobalData {
    pub content: String, // todo: can be all sorts of bytes
}

#[derive(Debug)]
pub enum RegisterValue {
    ByteLocation(usize),
    Int(i32),
}
