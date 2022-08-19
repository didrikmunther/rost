use crate::compiler::definition::{Procedure, SystemCall};

use super::{
    error::{NasmError, NasmErrorKind},
    generator::Generator,
    row::Row,
};

static ARG_REG: &[&'static str] = &["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

impl<'a> Generator<'a> {
    pub fn system_call(
        &mut self,
        procedure: &Procedure,
        system_call: &SystemCall,
    ) -> Result<(), NasmError> {
        let nargs = system_call.nargs;

        if nargs > ARG_REG.len() {
            return Err(NasmError::new(
                procedure.pos.clone(),
                NasmErrorKind::TooManyArguments(nargs),
            ));
        }

        for i in 0..nargs {
            let reg = ARG_REG[nargs - i - 1];
            self.code.add(Row::Pop(reg.into()));
        }

        self.code
            .add(Row::Xor("rax".into(), "rax".into()))
            .add(Row::Call(system_call.identifier.clone()));

        Ok(())
    }
}
