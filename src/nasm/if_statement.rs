use crate::compiler::definition::If;

use super::{error::NasmError, generator::Generator, row::Row};

impl<'a> Generator<'a> {
    pub fn handle_if_statement(
        &mut self,
        procedure: &str,
        if_statement: &If,
    ) -> Result<(), NasmError> {
        let label = Self::get_procedure_name(procedure, Some("if"));

        self.code
            .add(Row::Pop("rax".into()))
            .add(Row::Compare("rax".into(), "1".into()))
            .add(Row::JumpIfNotEquals(label.clone()));

        self.add_program(&if_statement.content, &label)?
            .add(Row::Label(label));

        Ok(())
    }
}
