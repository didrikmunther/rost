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
                StatementKind::IfStatements(if_statements) => {
                    self.handle_if_statement(statement, if_statements)
                }
                StatementKind::WhileStatement(while_statement) => {
                    self.handle_while_statement(statement, while_statement)
                }
                _ => todo!(),
            },
            DeclarationKind::FunctionDeclaration(fn_declaration) => {
                self.handle_function_declaration(fn_declaration)
            }
        }
    }
}
