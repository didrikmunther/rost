use crate::compiler::definition::OperandValue;

use super::{error::NasmError, generator::Generator, row::Row};

impl<'a> Generator<'a> {
    pub fn handle_push(&mut self, operand: &OperandValue) -> Result<(), NasmError> {
        match operand {
            OperandValue::Int(i) => self.code.add(Row::Push(format!("dword {}", *i))),
            OperandValue::StackLocation(loc) => self
                .code
                .add(Row::Move(
                    "rcx".into(),
                    self.get_absolute_stack_location(*loc),
                ))
                .add(Row::Push("rcx".into())),
            OperandValue::DataLocation(label) => self
                .code
                .add(Row::Move("rcx".into(), format!("[{}]", label)))
                .add(Row::Push("rcx".into())),
            OperandValue::DataPointerLocation(label) => {
                self.code.add(Row::Push(format!("{}", label)))
            }
        };

        Ok(())
    }
}
