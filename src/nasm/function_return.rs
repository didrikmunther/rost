use crate::compiler::definition::Procedure;

use super::{error::NasmError, generator::Generator, row::Row};

impl<'a> Generator<'a> {
    pub fn handle_function_return(&mut self, _procedure: &Procedure) -> Result<(), NasmError> {
        let offset = self.code.stack_pos - self.code.function_start_pos.unwrap();

        self.code
            .add(Row::Move("rax".into(), "[rsp]".into()))
            .add_with_comment(
                Row::Add("rsp".into(), format!("{}", offset * 8).into()),
                "Restoring stack pointer for return".into(),
            )
            .add(Row::Ret);

        Ok(())
    }
}
