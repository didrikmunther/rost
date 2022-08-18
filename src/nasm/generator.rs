use crate::compiler::{
    definition::{Arithmetic, OperandValue, ProcedureKind},
    program::Program,
};

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
            self.code.add(Row::Comment(format!(
                "[procedure {}]: {:?}",
                i, procedure.kind
            )));

            match &procedure.kind {
                ProcedureKind::SystemCall(system_call) => {
                    self.system_call(procedure, system_call)?
                }
                ProcedureKind::Push(operand) => self.handle_push(operand)?,
                ProcedureKind::Arithmetic(arithmetic) => self.handle_arithmetic(arithmetic)?,
            }
        }

        return Ok(&mut self.code);
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
            _ => todo!(),
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
        self.code
            .add(Row::Comment("[exit program]".into()))
            .add(Row::Ret)
    }
}
