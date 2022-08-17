use crate::compiler::{
    definition::{Procedure, ProcedureKind, RegisterValue},
    program::Program,
};

use super::{
    code::Code,
    error::{NasmError, NasmErrorKind},
    row::Row,
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

    pub fn generate_code(mut self) -> Result<Code, NasmError> {
        self.add_header();
        self.add_program()?;
        self.add_exit();
        self.add_data();

        Ok(self.code)
    }

    fn system_call(
        &mut self,
        procedure: &Procedure,
        args: &Vec<RegisterValue>,
    ) -> Result<(), NasmError> {
        if let Some(format) = args.get(0) {
            let value = match format {
                RegisterValue::ByteLocation(i) => Self::get_data_name(*i),
                RegisterValue::Int(_) => {
                    return Err(NasmError::new(
                        procedure.pos.clone(),
                        NasmErrorKind::InvalidArgumentType("int".into()),
                    ))
                }
            };

            self.code.add(Row::Move("rdi".into(), value));
        } else {
            return Err(NasmError::new(
                procedure.pos.end..procedure.pos.end + 1,
                NasmErrorKind::InvalidArgumentType("void".into()),
            )); // todo: type system
        }

        if let Some(value) = args.get(1) {
            let value = match value {
                RegisterValue::ByteLocation(i) => Self::get_data_name(*i),
                RegisterValue::Int(i) => i.to_string(),
            };

            self.code.add(Row::Move("rsi".into(), value));
        } else {
            return Err(NasmError::new(
                procedure.pos.end..procedure.pos.end + 1,
                NasmErrorKind::InvalidArgumentType("void".into()),
            ));
        }

        self.code
            .add(Row::Xor("rax".into(), "rax".into()))
            .add(Row::Call("printf".into()));

        Ok(())
    }

    fn add_program(&mut self) -> Result<&mut Code, NasmError> {
        for (i, procedure) in self.program.procedures.iter().enumerate() {
            self.code.add(Row::Comment(format!("[procedure {}]", i)));

            match &procedure.kind {
                ProcedureKind::SystemCall(system_call) => {
                    self.system_call(procedure, &system_call.args)?;
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
