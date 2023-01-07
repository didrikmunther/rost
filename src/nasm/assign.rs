use super::{error::NasmError, generator::Generator, row::Row};

impl<'a> Generator<'a> {
    pub fn handle_assign(&mut self, loc: usize) -> Result<(), NasmError> {
        self.code
            .add(Row::Pop("rax".into()))
            .add(Row::Move(format!("[rbp-{}]", (loc + 1) * 8), "rax".into()));

        Ok(())
    }
}
