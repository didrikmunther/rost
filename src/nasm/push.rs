use crate::compiler::definition::OperandValue;

use super::{error::NasmError, generator::Generator, row::Row};

impl<'a> Generator<'a> {
    pub fn handle_push(&mut self, operand: &OperandValue) -> Result<(), NasmError> {
        match operand {
            OperandValue::ByteLocation(loc) => self.code.add(Row::Push(Self::get_data_name(*loc))),
            OperandValue::Int(i) => self.code.add(Row::Push(format!("dword {}", *i))),
            OperandValue::StackLocation(loc) => self
                .code
                .add(Row::Move("rcx".into(), format!("[rbp-{}]", (loc + 1) * 8)))
                .add(Row::Push("rcx".into())),
        };

        Ok(())
    }
}
