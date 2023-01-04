use crate::compiler::{builder::Builder, definition::ProcedureKind, program::Program};

use super::{code::Code, error::NasmError, row::Row};

pub struct Generator<'a> {
    pub code: Code,
    pub program: &'a Program,
    pub output_comments: bool,
    pub optimize: bool,
    pub alignment: usize,
}

impl<'a> Generator<'a> {
    pub fn new(program: &'a Program) -> Self {
        Self {
            code: Code::new(),
            program,
            output_comments: false,
            optimize: false,
            alignment: 0
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
        self.add_program(&self.program.procedures, "")?;
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

    pub fn add_program(
        &mut self,
        procedures: &Builder,
        label_prefix: &str,
    ) -> Result<&mut Code, NasmError> {
        for (i, procedure) in procedures.iter().enumerate() {
            let label = format!("{}_{}", label_prefix, i);

            self.code.add(Row::Comment(format!(
                "[procedure {}]: {}",
                label, procedure.kind
            )));

            match &procedure.kind {
                ProcedureKind::Comment(comment) => {
                    self.code.add(Row::Comment(comment.clone()));
                }
                ProcedureKind::SystemCall(system_call) => {
                    self.handle_system_call(procedure, system_call)?
                }
                ProcedureKind::Reassign(reassign) => {
                    self.handle_reassign(*reassign)?;
                }
                ProcedureKind::Push(operand) => self.handle_push(operand)?,
                ProcedureKind::Arithmetic(arithmetic) => {
                    self.handle_arithmetic(&label, arithmetic)?
                }
                ProcedureKind::If(ifs) => self.handle_if_statement(&label, ifs)?,
                ProcedureKind::While(while_statement) => {
                    self.handle_while_statement(&label, while_statement)?
                }
            };
        }

        return Ok(&mut self.code);
    }

    pub fn get_procedure_name(i: &str, addition: Option<&str>) -> String {
        format!("_procedure_{}_{}", i, addition.unwrap_or(""))
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
