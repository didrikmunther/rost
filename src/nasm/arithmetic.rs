use crate::compiler::definition::Arithmetic;

use super::{error::NasmError, generator::Generator, row::Row};

impl<'a> Generator<'a> {
    pub fn handle_arithmetic(&mut self, arithmetic: &Arithmetic) -> Result<(), NasmError> {
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
}
