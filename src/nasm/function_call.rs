use std::ops::Range;

use crate::compiler::definition::{Procedure, ProcedureCall, SystemCall};

use super::{
    error::{NasmError, NasmErrorKind},
    generator::Generator,
    row::Row,
};

static ARG_REG: &[&'static str] = &["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

impl<'a> Generator<'a> {
    fn push_args(&mut self, nargs: usize, pos: Range<usize>) -> Result<(), NasmError> {
        if nargs > ARG_REG.len() {
            return Err(NasmError::new(pos, NasmErrorKind::TooManyArguments(nargs)));
        }

        for i in 0..nargs {
            let reg = ARG_REG[nargs - i - 1];
            self.code.add(Row::Pop(reg.into()));
        }

        Ok(())
    }

    pub fn handle_system_call(
        &mut self,
        procedure: &Procedure,
        call: &SystemCall,
    ) -> Result<(), NasmError> {
        self.push_args(call.nargs, procedure.pos.clone())?;

        self.code.aligned(|code| {
            code.add(Row::Xor("rax".into(), "rax".into()))
                .add(Row::Call(call.identifier.clone()))
        });

        Ok(())
    }

    pub fn handle_procedure_call(
        &mut self,
        _procedure: &Procedure,
        call: &ProcedureCall,
    ) -> Result<(), NasmError> {
        self.code
            .add(Row::Xor("rax".into(), "rax".into())) // Return value in rax, default 0
            .add(Row::Call(Self::get_function_name(call.function_id)));

        if call.returns {
            self.code.add(Row::Push("rax".into()));
        }

        Ok(())
    }
}
