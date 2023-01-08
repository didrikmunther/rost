use crate::compiler::definition::OperandValue;

use super::{error::NasmError, generator::Generator, row::Row};

impl<'a> Generator<'a> {
    pub fn handle_deref(&mut self) -> Result<(), NasmError> {
        self.code
            .add(Row::Pop("rcx".into()))
            .add(Row::Move("rcx".into(), "[rcx]".into()))
            .add(Row::Push("rcx".into()));

        Ok(())
    }

    /// Pushes the value to the stack.
    /// If `push_address`, use LEA instead of `MOV`.
    pub fn handle_push(
        &mut self,
        operand: &OperandValue,
        push_address: bool,
    ) -> Result<(), NasmError> {
        match operand {
            OperandValue::Int(i) => {
                if push_address {
                    todo!("Unsupported");
                }

                self.code.add(Row::Push(format!("dword {}", *i)))
            }
            OperandValue::StackLocation(loc) => {
                if push_address {
                    self.code.add(Row::LoadEffectiveAddress(
                        "rcx".into(),
                        self.get_absolute_stack_location(*loc),
                    ));
                } else {
                    self.code.add(Row::Move(
                        "rcx".into(),
                        self.get_absolute_stack_location(*loc),
                    ));
                }

                self.code.add(Row::Push("rcx".into()))
            }
            OperandValue::DataLocation(label) => {
                if push_address {
                    self.code.add(Row::LoadEffectiveAddress(
                        "rcx".into(),
                        format!("[{}]", label),
                    ));
                } else {
                    self.code
                        .add(Row::Move("rcx".into(), format!("[{}]", label)));
                }

                self.code.add(Row::Push("rcx".into()))
            }
            OperandValue::DataPointerLocation(label) => {
                self.code.add(Row::Push(format!("dword {}", label)))
            }
        };

        Ok(())
    }
}
