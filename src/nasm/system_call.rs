use crate::compiler::definition::{OperandValue, Procedure};

use super::{
    error::{NasmError, NasmErrorKind},
    generator::Generator,
    row::Row,
};

impl<'a> Generator<'a> {
    pub fn system_call(
        &mut self,
        procedure: &Procedure,
        args: &Vec<OperandValue>,
    ) -> Result<(), NasmError> {
        if let Some(format) = args.get(0) {
            let value = match format {
                OperandValue::ByteLocation(i) => Self::get_data_name(*i),
                _ => {
                    return Err(NasmError::new(
                        procedure.pos.clone(),
                        NasmErrorKind::InvalidArgumentType("?".into()),
                    ))
                }
            };

            self.code.add(Row::Move("rdi".into(), value));
        } else {
            return Err(NasmError::new(
                procedure.pos.end..procedure.pos.end + 1,
                NasmErrorKind::InvalidArgumentType("void".into()),
            )); // todo: type system
        }

        if let Some(value) = args.get(1) {
            let value = match value {
                OperandValue::ByteLocation(i) => Self::get_data_name(*i),
                OperandValue::StackLocation(i) => {
                    format!("[rsp+{}]", (self.program.stack_pos - i) * 8)
                }
                OperandValue::Int(i) => i.to_string(),
            };

            self.code.add(Row::Move("rsi".into(), value));
        } else {
            return Err(NasmError::new(
                procedure.pos.end..procedure.pos.end + 1,
                NasmErrorKind::InvalidArgumentType("void".into()),
            ));
        }

        self.code
            .add(Row::Xor("rax".into(), "rax".into()))
            .add(Row::Call("printf".into()));

        Ok(())
    }
}
