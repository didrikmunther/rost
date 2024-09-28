use crate::compiler::definition::Procedure;

use super::{error::NasmError, generator::Generator, row::Row};

impl<'a> Generator<'a> {
    pub fn handle_function_return(&mut self, _procedure: &Procedure) -> Result<(), NasmError> {
        self.code.add(Row::Move("rax".into(), "[rsp]".into()));

        self.restore_base_pointer().add(Row::Ret);

        Ok(())
    }
}
