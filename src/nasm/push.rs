use crate::compiler::definition::OperandValue;

use super::{error::NasmError, generator::Generator, row::Row};

impl<'a> Generator<'a> {
    pub fn handle_push(&mut self, operand: &OperandValue) -> Result<(), NasmError> {
        match operand {
            OperandValue::ByteLocation(loc) => self.code.add(Row::Push(Self::get_data_name(*loc))),
            OperandValue::Int(i) => self.code.add(Row::Push(format!("{}", *i))),
            OperandValue::StackLocation(loc) => self
                .code
                .add_with_stack(|stack_pos| {
                    Row::Move(
                        "rcx".into(),
                        format!("[rsp+{}]", (stack_pos - *loc - 1) * 8),
                    )
                })
                .add(Row::Push("rcx".into())),
        };

        Ok(())
    }
}
