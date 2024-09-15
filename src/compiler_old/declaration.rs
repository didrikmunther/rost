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
                StatementKind::VariableAssignment(assignment) => self.handle_variable_assignment(assignment),
                StatementKind::VariableDeclaration(declaration) => self.handle_variable_declaration(declaration),
                StatementKind::IfStatements(if_statements) => {
                    self.handle_if_statement(statement, if_statements)
                }
                StatementKind::WhileStatement(while_statement) => {
                    self.handle_while_statement(statement, while_statement)
                }
                StatementKind::ReturnStatement(ret_statement) => self.handle_return_statement(statement, ret_statement),
            },
            DeclarationKind::FunctionDeclaration(fn_declaration) => {
                self.handle_function_declaration(declaration, fn_declaration)
            }
            DeclarationKind::StructDeclaration(struct_declaration) => {
                self.handle_struct_declaration(declaration, struct_declaration)
            }
        }
    }
}
