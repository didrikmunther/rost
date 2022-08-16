use std::{
    fmt::{Arguments, Display, Formatter},
    mem::swap,
};

use crate::parser::Declaration;

#[derive(Debug)]
pub struct CompilerError;

#[derive(Debug)]
pub struct CodeRow {
    row: Row,
    newline: bool,
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

#[derive(Debug)]
pub struct Code {
    rows: Vec<CodeRow>,
}

struct Compiler {
    code: Code,
}

impl Code {
    pub fn new() -> Self {
        Self { rows: Vec::new() }
    }

    pub fn add(&mut self, row: Row) -> &mut Self {
        self.rows.push(CodeRow { row, newline: true });
        self
    }

    pub fn add_with_comment(&mut self, row: Row, comment: String) -> &mut Self {
        self.rows.push(CodeRow {
            row,
            newline: false,
        });

        self.rows.push(CodeRow {
            row: Row::Comment(comment),
            newline: true,
        });
        self
    }
}

impl Display for Code {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for row in &self.rows {
            fmt.write_fmt(format_args!("{}", row))?
        }

        Ok(())
    }
}

impl Compiler {
    pub fn new() -> Self {
        Self { code: Code::new() }
    }

    pub fn compile(
        &mut self,
        parsed: &Vec<Declaration>,
    ) -> Result<Code, CompilerError> {
        self.add_header()
            .add_with_comment(
                Row::Move("rax".into(), "1".into()),
                "system call for write".into(),
            )
            .add(Row::Move("rdi".into(), "1".into()))
            .add(Row::Move("rsi".into(), "message".into()))
            .add(Row::Move("rdx".into(), "13".into()))
            .add(Row::Syscall);

        self.add_exit()
            .add(Row::Section("data".into()))
            .add(Row::Label("message".into()))
            .add(Row::DeclareStaticString("Hello, World".into()));

        let mut code = Code::new();
        swap(&mut code, &mut self.code);
        Ok(code)
    }

    fn add_exit(&mut self) -> &mut Code {
        self.code
            .add_with_comment(
                Row::Move("rax".into(), "60".into()),
                "system call for exit".into(),
            )
            .add(Row::Xor("rdi".into(), "rdi".into()))
            .add(Row::Syscall)
    }

    fn add_header(&mut self) -> &mut Code {
        self.code
            .add(Row::Global("_start".into()))
            .add(Row::Section("text".into()))
            .add(Row::Label("_start".into()))
    }
}

pub fn compile(parsed: &Vec<Declaration>) -> Result<Code, CompilerError> {
    Compiler::new().compile(parsed)
}
