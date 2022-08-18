use crate::compiler::{definition::ProcedureKind, program::Program};

use super::{code::Code, error::NasmError, row::Row};

pub struct Generator<'a> {
    pub code: Code,
    pub program: &'a Program,
}

impl<'a> Generator<'a> {
    pub fn new(program: &'a Program) -> Self {
        Self {
            code: Code::new(),
            program,
        }
    }

    pub fn generate_code(mut self) -> Result<Code, NasmError> {
        self.add_header();
        self.add_program()?;
        self.add_exit();
        self.add_data();

        Ok(self.code)
    }

    fn add_program(&mut self) -> Result<&mut Code, NasmError> {
        for (i, procedure) in self.program.procedures.iter().enumerate() {
            self.code.add(Row::Comment(format!("[procedure {}]", i)));

            match &procedure.kind {
                ProcedureKind::SystemCall(system_call) => {
                    self.system_call(procedure, &system_call.args)?;
                }
                ProcedureKind::Assignment(assignment) => {
                    self.assignment(procedure, assignment)?;
                }
                _ => todo!(),
            }
        }

        return Ok(&mut self.code);
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
        self.code
            .add(Row::Comment("[data]".into()))
            .add(Row::Section("data".into()));

        for (i, data) in self.program.global_data.iter().enumerate() {
            self.code
                .add(Row::Label(Self::get_data_name(i)))
                .add(Row::DeclareStaticString(data.content.clone()));
        }

        &mut self.code
    }

    fn add_exit(&mut self) -> &mut Code {
        self.code
            .add(Row::Comment("[exit]".into()))
            .add_with_comment(Row::Ret, "[exit program]".into())
    }
}
