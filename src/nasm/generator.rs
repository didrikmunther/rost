use crate::compiler::{builder::Builder, definition::ProcedureKind, program::Program};

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
        self.add_program(&self.program.procedures, "")?;
        self.add_exit();
        self.add_functions()?;
        self.add_data();

        if self.optimize {
            let mut did_remove = true;
            let mut total_removed = 0;
            let mut passes = 0;

            while did_remove {
                let (code, removed) = self.code.optimized();
                self.code = code;
                did_remove = removed > 0;
                total_removed += removed;
                passes += 1;
            }

            println!(
                "[NASM Optimizer]: Removed {total_removed} lines in {} passes",
                passes - 1
            );
        }

        if !self.output_comments {
            self.code = self.code.strip_comments();
        }

        Ok(self.code)
    }

    pub fn add_block<F>(&mut self, inner: F) -> Result<&mut Code, NasmError>
    where
        F: FnOnce(&mut Self) -> Result<(), NasmError>,
    {
        let old_stack_pos = self.code.stack_pos;

        inner(self)?;

        self.code.add_with_comment(
            Row::Add(
                "rsp".into(),
                format!("{}", (self.code.stack_pos - old_stack_pos) * 8).into(),
            ),
            "Restoring stack pointer".into(),
        );

        self.code.stack_pos = old_stack_pos;

        Ok(&mut self.code)
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
                ProcedureKind::Allocate(size) => {
                    self.code
                        .add(Row::Subtract("rsp".into(), format!("{}", size * 8)));
                }
                ProcedureKind::SystemCall(system_call) => {
                    self.handle_system_call(procedure, system_call)?
                }
                ProcedureKind::ProcedureCall(procedure_call) => {
                    self.handle_procedure_call(procedure, procedure_call)?
                }
                ProcedureKind::Return => self.handle_function_return(procedure)?,
                ProcedureKind::Assign(location) => self.handle_assign(location)?,
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

        Ok(&mut self.code)
    }

    pub fn get_procedure_name(i: &str, addition: Option<&str>) -> String {
        format!("_procedure_{}_{}", i, addition.unwrap_or(""))
    }

    pub fn get_function_name(function_id: usize) -> String {
        format!("_function_{}", function_id)
    }

    pub fn get_absolute_stack_location(&self, loc: isize) -> String {
        format!("[rbp{}{}]", if loc < 0 { '+' } else { '-' }, loc.abs() * 8)
    }

    fn add_header(&mut self) -> &mut Code {
        self.code
            .add(Row::Comment("[header]".into()))
            .add(Row::Global("main".into()))
            .add(Row::Extern("printf".into()))
            .add(Row::Section("text".into()))
            .add(Row::Label("main".into()))
            .add(Row::Comment("Save base pointer".into()))
            .add(Row::Push("rbp".into()))
            .add(Row::Move("rbp".into(), "rsp".into()))
    }

    fn add_exit(&mut self) -> &mut Code {
        self.code
            .add(Row::Comment("[exit program]".into()))
            .add(Row::Comment("Restore base pointer".into()))
            .add(Row::Move("rsp".into(), "rbp".into()))
            .add(Row::Pop("rbp".into()))
            .add(Row::Ret)
    }

    fn add_functions(&mut self) -> Result<&mut Code, NasmError> {
        self.code
            .add(Row::Comment("[enter function definitions]".into()));

        for (i, function) in self.program.functions.iter().enumerate() {
            self.handle_function_declaration(i, function)?;
        }

        Ok(&mut self.code)
    }
}
