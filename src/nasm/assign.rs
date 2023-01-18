use crate::compiler::{definition::Assign, scope::variable::VariableLocation};

use super::{error::NasmError, generator::Generator, row::Row};

impl<'a> Generator<'a> {
    pub fn handle_assign(&mut self, assign: &Assign) -> Result<(), NasmError> {
        for i in 0..assign.size / 8 {
            self.code.add(Row::Pop("rax".into()));

            let into = match &assign.location {
                VariableLocation::Stack(loc) => self.get_absolute_stack_location(*loc + i as isize),
                VariableLocation::Global(label) => format!("[{}]", label),
                VariableLocation::Address => {
                    self.code.add(Row::Pop("rbx".into()));

                    "[rbx]".into()
                }
            };

            self.code.add(Row::Move(into, "rax".into()));
        }

        Ok(())
    }
}
