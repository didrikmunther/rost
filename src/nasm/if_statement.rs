use crate::compiler::definition::If;

use super::{error::NasmError, generator::Generator, row::Row};

impl<'a> Generator<'a> {
    pub fn handle_if_statement(
        &mut self,
        procedure: &str,
        if_statements: &[If],
    ) -> Result<(), NasmError> {
        let label_end = Self::get_procedure_name(procedure, Some("if_end"));

        for (i, if_statement) in if_statements.iter().enumerate() {
            let label_condition =
                Self::get_procedure_name(procedure, Some(&format!("if_condition_{i}")));
            let label_content =
                Self::get_procedure_name(procedure, Some(&format!("if_content_{i}")));

            if let Some(condition) = &if_statement.condition {
                // If / else-if statement
                self.add_block(|generator| {
                    generator
                        .add_program(condition, &label_condition)?
                        .add(Row::Pop("rax".into()));

                    Ok(())
                })?;
            } else {
                // Else statement
                let label_content = Self::get_procedure_name(procedure, Some("else_content"));

                self.add_block(|generator| {
                    generator.add_program(&if_statement.content, &label_content)?;
                    Ok(())
                })?;

                break;
            }

            self.code
                .add(Row::Compare("rax".into(), "1".into()))
                .add(Row::JumpIfNotEquals(label_content.clone()));

            self.add_block(|generator| {
                generator.add_program(&if_statement.content, &label_content)?;
                Ok(())
            })?;

            self.code
                .add(Row::Jump(label_end.clone()))
                .add(Row::Label(label_content));
        }

        self.code.add(Row::Label(label_end));

        Ok(())
    }
}
