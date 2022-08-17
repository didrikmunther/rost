use crate::parser::definition::{Declaration, DeclarationKind, StatementKind};

use super::{
    definition::{Procedure},
    error::{CompilerError},
    program::Program,
};

impl Program {
    pub fn handle_declaration(
        &mut self,
        declaration: &Declaration,
    ) -> Result<Procedure, CompilerError> {
        match &declaration.kind {
            DeclarationKind::Statement(statement) => match &statement.kind {
                StatementKind::Assignment(assignment) => self.handle_assignment(assignment),
                StatementKind::Expression(expression) => self.handle_expression(expression),
            },
        }
    }
}
