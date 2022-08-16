use std::mem::swap;

use super::{code::Code, error::CompilerError, program::Program, row::Row};

pub struct ASMGenererator<'a> {
    code: Code,
    program: &'a Program,
}

impl<'a> ASMGenererator<'a> {
    pub fn new(program: &'a Program) -> Self {
        Self {
            code: Code::new(),
            program,
        }
    }

    pub fn generate_code(&mut self) -> Result<Code, CompilerError> {
        // Init
        self.add_header();

        // Hello world
        self.code
            .add_with_comment(
                Row::Move("rax".into(), "1".into()),
                "system call for write".into(),
            )
            .add(Row::Move("rdi".into(), "1".into()))
            .add(Row::Move("rsi".into(), "message".into()))
            .add(Row::Move("rdx".into(), "13".into()))
            .add(Row::Syscall);

        // Exit code
        self.add_exit();

        // Data
        self.code
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
