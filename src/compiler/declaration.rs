use crate::parser::definition::{Declaration, DeclarationKind, StatementKind};

use super::{builder::Builder, error::CompilerError, program::Program};

impl Program {
    pub fn handle_declaration(
        &mut self,
        declaration: &Declaration,
    ) -> Result<Builder, CompilerError> {
        match &declaration.kind {
            DeclarationKind::Statement(statement) => match &statement.kind {
                StatementKind::Expression(expression) => self.handle_expression(expression),
                StatementKind::Assignment(assignment) => self.handle_assignment(assignment),
            },
        }
    }
}
