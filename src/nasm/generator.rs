use std::mem::swap;

use crate::compiler::program::{ProcedureKind, Program};

use super::{
    code::Code,
    error::{NasmError, NasmErrorKind},
    row::Row,
    system_call::SYSTEM_CALLS,
};

pub struct Generator<'a> {
    code: Code,
    program: &'a Program,
}

impl<'a> Generator<'a> {
    pub fn new(program: &'a Program) -> Self {
        Self {
            code: Code::new(),
            program,
        }
    }

    pub fn generate_code(&mut self) -> Result<Code, NasmError> {
        self.add_header();
        self.add_program()?;
        self.add_exit();
        self.add_data();

        let mut code = Code::new();
        swap(&mut code, &mut self.code);
        Ok(code)
    }

    fn get_system_call(&mut self, id: usize, arg: String) -> &mut Code {
        self.code
            .add_with_comment(
                Row::Move("rax".into(), format!("{}", id)),
                "system call for write".into(),
            )
            .add(Row::Move("rdi".into(), "1".into()))
            .add(Row::Move("rsi".into(), arg))
            .add(Row::Move("rdx".into(), "13".into()))
            .add(Row::Syscall)
    }

    fn add_program(&mut self) -> Result<&mut Code, NasmError> {
        for (i, procedure) in self.program.procedures.iter().enumerate() {
            self.code.add(Row::Comment(format!("[procedure {}]", i)));

            match &procedure.kind {
                ProcedureKind::SystemCall(system_call) => {
                    if let Some(&system_call_id) = SYSTEM_CALLS.get(&system_call.identifier) {
                        self.get_system_call(
                            system_call_id,
                            Self::get_data_name(*system_call.args.get(0).unwrap()),
                        );
                    } else {
                        return Err(NasmError::new(
                            procedure.pos.clone(),
                            NasmErrorKind::UnknownSystemCall(system_call.identifier.clone()),
                        ));
                    }
                }
            }
        }

        return Ok(&mut self.code);
    }

    fn get_data_name(i: usize) -> String {
        format!("_data_{}", i)
    }

    fn add_header(&mut self) -> &mut Code {
        self.code
            .add(Row::Comment("[header]".into()))
            .add(Row::Global("_start".into()))
            .add(Row::Section("text".into()))
            .add(Row::Label("_start".into()))
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
            .add_with_comment(
                Row::Move("rax".into(), "60".into()),
                "system call for exit".into(),
            )
            .add(Row::Xor("rdi".into(), "rdi".into()))
            .add(Row::Syscall)
    }
}
