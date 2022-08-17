use std::fmt::{Arguments, Display, Formatter};

#[derive(Debug)]
pub struct CodeRow {
    row: Row,
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

#[derive(Debug)]
pub enum Row {
    Comment(String),
    Move(String, String), // todo: types for numbers, registers, or labels
    Xor(String, String),
    Section(String),
    Label(String),
    Global(String),
    // DeclareByte(String), // todo: allow for all allowed values: https://www.nasm.us/doc/nasmdoc3.html 3.2.1
    DeclareStaticString(String), // Declare byte abstraction, completes it with the string length
    Syscall,
}

impl Display for Row {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut w = |args: Arguments| fmt.write_fmt(args);

        match self {
            Row::Comment(comment) => w(format_args!("\t; {}", *comment)),
            Row::Move(to, from) => w(format_args!("\tmov {}, {}", to, from)),
            Row::Xor(to, from) => w(format_args!("\txor {}, {}", to, from)),
            Row::Section(section) => w(format_args!("\n\tsection .{}", section)),
            Row::Label(label) => w(format_args!("{}:", label)),
            Row::Global(global) => w(format_args!("\tglobal {}", global)),
            Row::DeclareStaticString(s) => w(format_args!("\tdb \"{}\", {}", s, s.len())),
            Row::Syscall => w(format_args!("\tsyscall")),
            _ => w(format_args!("; UNKNOWN")),
        }
    }
}