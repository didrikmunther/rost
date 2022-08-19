use super::{error::NasmError, generator::Generator, row::Row};

impl<'a> Generator<'a> {
    pub fn handle_reassign(&mut self, loc: usize) -> Result<(), NasmError> {
        self.code
            .add(Row::Pop("rax".into()))
            .add_with_stack(|stack_pos| {
                Row::Move(format!("[rsp+{}]", (stack_pos - loc - 1) * 8), "rax".into())
            });

        Ok(())
    }
}
