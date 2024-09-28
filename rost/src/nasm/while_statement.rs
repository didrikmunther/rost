use crate::compiler::definition::While;

use super::{error::NasmError, generator::Generator, row::Row};

impl<'a> Generator<'a> {
    pub fn handle_while_statement(
        &mut self,
        procedure: &str,
        while_statement: &While,
    ) -> Result<(), NasmError> {
        let label_condition = Self::get_procedure_name(procedure, Some("while_condition"));
        let label_content = Self::get_procedure_name(procedure, Some("while_content"));
        let label_content_end = Self::get_procedure_name(procedure, Some("while_content_end"));

        self.code.add(Row::Label(label_condition.clone()));
        self.add_block(|generator| {
            generator
                .add_program(&while_statement.condition, &label_condition)?
                .add(Row::Pop("rax".into()))
                .add(Row::Compare("rax".into(), "1".into()))
                .add(Row::JumpIfNotEquals(label_content_end.clone()));

            Ok(())
        })?;

        self.add_block(|generator| {
            generator.add_program(&while_statement.content, &label_content)?;

            Ok(())
        })?;

        self.code
            .add(Row::Jump(label_condition))
            .add(Row::Label(label_content_end));

        Ok(())
    }
}
