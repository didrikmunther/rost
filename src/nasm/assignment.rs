use crate::compiler::definition::{Assignment, OperandValue, Procedure};

use super::{
    error::{NasmError, NasmErrorKind},
    generator::Generator,
    row::Row,
};

impl<'a> Generator<'a> {
    pub fn assignment(
        &mut self,
        procedure: &Procedure,
        assignment: &Assignment,
    ) -> Result<(), NasmError> {
        

        todo!()
    }
}
