use crate::compiler::definition::{OperandValue, Procedure, SystemCall};

use super::{
    error::{NasmError, NasmErrorKind},
    generator::Generator,
    row::Row,
};

static ARG_REG: &[&'static str] = &["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

impl<'a> Generator<'a> {
    pub fn system_call(
        &mut self,
        _procedure: &Procedure,
        system_call: &SystemCall,
    ) -> Result<(), NasmError> {
        let nargs = system_call.nargs;

        for i in 0..nargs {
            let reg = ARG_REG[nargs - i - 1];
            self.code.add(Row::Pop(reg.into()));
        }

        self.code
            .add(Row::Xor("rax".into(), "rax".into()))
            .add(Row::Call(system_call.identifier.clone()));

        Ok(())

        // if let Some(format) = args.get(0) {
        //     let value = match format {
        //         OperandValue::ByteLocation(i) => Self::get_data_name(*i),
        //         _ => {
        //             return Err(NasmError::new(
        //                 procedure.pos.clone(),
        //                 NasmErrorKind::InvalidArgumentType("?".into()),
        //             ))
        //         }
        //     };

        //     self.code.add(Row::Move("rdi".into(), value));
        // } else {
        //     return Err(NasmError::new(
        //         procedure.pos.end..procedure.pos.end + 1,
        //         NasmErrorKind::InvalidArgumentType("void".into()),
        //     )); // todo: type system
        // }

        // if let Some(value) = args.get(1) {
        //     let value = match value {
        //         OperandValue::ByteLocation(i) => Self::get_data_name(*i),
        //         OperandValue::StackLocation(i) => {
        //             format!("[rsp+{}]", (self.program.stack_pos - i) * 8)
        //         }
        //         OperandValue::Int(i) => i.to_string(),
        //     };

        //     self.code.add(Row::Move("rsi".into(), value));
        // } else {
        //     return Err(NasmError::new(
        //         procedure.pos.end..procedure.pos.end + 1,
        //         NasmErrorKind::InvalidArgumentType("void".into()),
        //     ));
        // }

        // self.code
        //     .add(Row::Xor("rax".into(), "rax".into()))
        //     .add(Row::Call("printf".into()));
    }
}
