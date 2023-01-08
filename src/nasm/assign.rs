use crate::compiler::scope::variable::VariableLocation;

use super::{error::NasmError, generator::Generator, row::Row};

impl<'a> Generator<'a> {
    pub fn handle_assign(&mut self, location: &VariableLocation) -> Result<(), NasmError> {
        self.code.add(Row::Pop("rax".into()));

        let into = match location {
            VariableLocation::Stack(loc) => self.get_absolute_stack_location(*loc),
            VariableLocation::Global(label) => format!("[{}]", label),
        };

        self.code.add(Row::Move(into, "rax".into()));

        Ok(())
    }
}
