use crate::compiler::definition::Arithmetic;

use super::{error::NasmError, generator::Generator, row::Row};

impl<'a> Generator<'a> {
    pub fn handle_arithmetic(
        &mut self,
        procedure: usize,
        arithmetic: &Arithmetic,
    ) -> Result<(), NasmError> {
        let operations = match arithmetic {
            Arithmetic::Add => vec![Row::Add("rax".into(), "rbx".into())],
            Arithmetic::Subtract => vec![Row::Subtract("rax".into(), "rbx".into())],
            Arithmetic::Multiply => vec![Row::Multiply("rbx".into())],
            Arithmetic::Equality => {
                let label = Self::get_procedure_name(procedure, Some("boolan_equality"));

                vec![
                    Row::Compare("rax".into(), "rbx".into()),
                    Row::Move("rax".into(), "1".into()),
                    Row::JumpIfEquals(label.clone()),
                    Row::Move("rax".into(), "0".into()),
                    Row::Label(label),
                ]
            }
            _ => todo!(),
        };

        self.code
            .add(Row::Pop("rax".into()))
            .add(Row::Pop("rbx".into()));

        for operation in operations {
            self.code.add(operation);
        }

        self.code.add(Row::Push("rax".into()));

        Ok(())
    }
}
