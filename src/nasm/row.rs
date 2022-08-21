use std::fmt::{Arguments, Display, Formatter};

#[derive(Debug, Clone)]
pub struct CodeRow {
    pub row: Row,
    newline: bool,
}

impl CodeRow {
    pub fn new(row: Row, newline: bool) -> Self {
        Self { row, newline }
    }
}

impl Display for CodeRow {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_fmt(format_args!("{}", self.row))?;

        if self.newline {
            fmt.write_str("\n")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Row {
    Comment(String),
    Extern(String),
    Move(String, String), // todo: types for numbers, registers, or labels
    Xor(String, String),
    Section(String),
    Label(String),
    Global(String),
    Call(String),
    Add(String, String),
    Subtract(String, String),
    Multiply(String),
    Compare(String, String),
    Jump(String),
    JumpIfEquals(String),
    JumpIfNotEquals(String),
    JumpIfLessThan(String),
    JumpIfGreaterThan(String),
    // DeclareByte(String), // todo: allow for all allowed values: https://www.nasm.us/doc/nasmdoc3.html 3.2.1
    DeclareStaticString(String), // Declare byte abstraction, completes it with the string length
    Push(String),
    Pop(String),
    Ret,
}

fn get_bytes(s: &String) -> String {
    s.as_bytes()
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

impl Display for Row {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut w = |args: Arguments| fmt.write_fmt(args);

        match self {
            Row::Comment(comment) => w(format_args!("\t; {}", *comment)),
            Row::Extern(ext) => w(format_args!("\textern {}", ext)),
            Row::Move(to, from) => w(format_args!("\tmov {}, {}", to, from)),
            Row::Xor(to, from) => w(format_args!("\txor {}, {}", to, from)),
            Row::Section(section) => w(format_args!("\n\tsection .{}", section)),
            Row::Label(label) => w(format_args!("{}:", label)),
            Row::Push(operand) => w(format_args!("\tpush {}", operand)),
            Row::Pop(operand) => w(format_args!("\tpop {}", operand)),
            Row::Add(to, from) => w(format_args!("\tadd {}, {}", to, from)),
            Row::Compare(to, from) => w(format_args!("\tcmp {}, {}", to, from)),
            Row::Jump(label) => w(format_args!("\tjmp {}", label)),
            Row::JumpIfEquals(label) => w(format_args!("\tje {}", label)),
            Row::JumpIfNotEquals(label) => w(format_args!("\tjne {}", label)),
            Row::JumpIfLessThan(label) => w(format_args!("\tjl {}", label)),
            Row::JumpIfGreaterThan(label) => w(format_args!("\tjg {}", label)),
            Row::Subtract(to, from) => w(format_args!("\tsub {}, {}", to, from)),
            Row::Multiply(to) => w(format_args!("\tmul {}", to)),
            Row::Global(global) => w(format_args!("\tglobal {}", global)),
            Row::Call(function) => w(format_args!("\tcall {}", function)),
            Row::DeclareStaticString(s) => w(format_args!("\tdb {}, 0", get_bytes(s))),
            Row::Ret => w(format_args!("\tret")),
        }
    }
}
