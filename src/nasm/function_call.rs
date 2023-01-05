use crate::compiler::definition::{Procedure, ProcedureCall};

use super::{
    error::{NasmError, NasmErrorKind},
    generator::Generator,
    row::Row,
};

static ARG_REG: &[&'static str] = &["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
static BUILT_IN: &[&'static str] = &["printf"];

impl<'a> Generator<'a> {
    pub fn handle_system_call(
        &mut self,
        procedure: &Procedure,
        call: &ProcedureCall,
    ) -> Result<(), NasmError> {
        let nargs = call.nargs;

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

        self.code.aligned(|code| {
            code.add(Row::Xor("rax".into(), "rax".into()))
                .add(Row::Call(call.identifier.clone()))
        });

        Ok(())
    }

    pub fn handle_function_call(
        &mut self,
        procedure: &Procedure,
        call: &ProcedureCall,
    ) -> Result<(), NasmError> {
        todo!()
    }

    pub fn handle_procedure_call(
        &mut self,
        procedure: &Procedure,
        call: &ProcedureCall,
    ) -> Result<(), NasmError> {
        if BUILT_IN.contains(&call.identifier.as_str()) {
            self.handle_system_call(procedure, call)
        } else {
            self.handle_function_call(procedure, call)
        }
    }
}
