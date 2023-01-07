use super::{error::NasmError, generator::Generator, row::Row};

impl<'a> Generator<'a> {
    pub fn handle_assign(&mut self, loc: isize) -> Result<(), NasmError> {
        let location = self.get_absolut_stack_location(loc);

        self.code
            .add(Row::Pop("rax".into()))
            .add(Row::Move(location, "rax".into()));

        Ok(())
    }
}
