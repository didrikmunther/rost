use crate::compiler::{
    definition::{Arithmetic, OperandValue, ProcedureKind},
    program::Program,
};

use super::{code::Code, error::NasmError, row::Row};

pub struct Generator<'a> {
    pub code: Code,
    pub program: &'a Program,
    pub output_comments: bool,
    pub optimize: bool,
}

impl<'a> Generator<'a> {
    pub fn new(program: &'a Program) -> Self {
        Self {
            code: Code::new(),
            program,
            output_comments: false,
            optimize: false,
        }
    }

    pub fn with_comments(mut self, output_comments: bool) -> Self {
        self.output_comments = output_comments;
        self
    }

    pub fn with_optimization(mut self, with_optimization: bool) -> Self {
        self.optimize = with_optimization;
        self
    }

    pub fn generate_code(mut self) -> Result<Code, NasmError> {
        self.add_header();
        self.add_program()?;
        self.add_exit();
        self.add_data();

        if self.optimize {
            let (code, removed) = self.code.optimized();
            self.code = code;

            println!("[NASM Optimizer]: Removed {} lines", removed);
        }

        if !self.output_comments {
            self.code = self.code.strip_comments();
        }

        Ok(self.code)
    }

    fn add_program(&mut self) -> Result<&mut Code, NasmError> {
        for (i, procedure) in self.program.procedures.iter().enumerate() {
            self.code.add(Row::Comment(format!(
                "[procedure {}]: {:?}",
                i, procedure.kind
            )));

            match &procedure.kind {
                ProcedureKind::Comment(comment) => {
                    self.code.add(Row::Comment(comment.clone()));
                }
                ProcedureKind::SystemCall(system_call) => {
                    self.system_call(procedure, system_call)?
                }
                ProcedureKind::Reassign(reassign) => {
                    self.reassign(*reassign)?;
                }
                ProcedureKind::Push(operand) => self.handle_push(operand)?,
                ProcedureKind::Arithmetic(arithmetic) => self.handle_arithmetic(arithmetic)?,
            };
        }

        return Ok(&mut self.code);
    }

    fn reassign(&mut self, loc: usize) -> Result<(), NasmError> {
        self.code
            .add(Row::Pop("rax".into()))
            .add_with_stack(|stack_pos| {
                Row::Move(format!("[rsp+{}]", (stack_pos - loc - 1) * 8), "rax".into())
            });

        Ok(())
    }

    fn handle_arithmetic(&mut self, arithmetic: &Arithmetic) -> Result<(), NasmError> {
        let operation = match arithmetic {
            Arithmetic::Add => Row::Add("rax".into(), "rbx".into()),
            Arithmetic::Subtract => Row::Subtract("rax".into(), "rbx".into()),
            Arithmetic::Multiply => Row::Multiply("rbx".into()),
        };

        self.code
            .add(Row::Pop("rax".into()))
            .add(Row::Pop("rbx".into()))
            .add(operation)
            .add(Row::Push("rax".into()));

        Ok(())
    }

    fn handle_push(&mut self, operand: &OperandValue) -> Result<(), NasmError> {
        match operand {
            OperandValue::ByteLocation(loc) => self.code.add(Row::Push(Self::get_data_name(*loc))),
            OperandValue::Int(i) => self.code.add(Row::Push(format!("{}", *i))),
            OperandValue::StackLocation(loc) => self
                .code
                .add_with_stack(|stack_pos| {
                    Row::Move(
                        "rcx".into(),
                        format!("[rsp+{}]", (stack_pos - *loc - 1) * 8),
                    )
                })
                .add(Row::Push("rcx".into())),
        };

        Ok(())
    }

    pub fn get_data_name(i: usize) -> String {
        format!("_data_{}", i)
    }

    fn add_header(&mut self) -> &mut Code {
        self.code
            .add(Row::Comment("[header]".into()))
            .add(Row::Global("main".into()))
            .add(Row::Extern("printf".into()))
            .add(Row::Section("text".into()))
            .add(Row::Label("main".into()))
    }

    fn add_data(&mut self) -> &mut Code {
        self.code.add(Row::Section("data".into()));

        for (i, data) in self.program.global_data.iter().enumerate() {
            self.code
                .add(Row::Label(Self::get_data_name(i)))
                .add(Row::DeclareStaticString(data.content.clone()));
        }

        &mut self.code
    }

    fn add_exit(&mut self) -> &mut Code {
        for i in 0..self.code.stack_pos {
            self.code.add_with_comment(
                Row::Pop("rax".into()),
                format!(
                    "Cleaning stack variable: {}",
                    self.program
                        .variables
                        .iter()
                        .find(|(_, v)| v.stack_pos == i)
                        .map(|(k, _)| k)
                        .unwrap_or(&String::from("unknown variable"))
                ),
            );
        }

        self.code
            .add(Row::Comment("[exit program]".into()))
            .add(Row::Ret)
    }
}
