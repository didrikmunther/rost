use super::{builder::Builder, Program};
use crate::{
    compiler::error::CompilerError,
    parser::definition::{Declaration, DeclarationKind, StatementKind},
};

impl Program {
    pub fn handle_declaration(
        &mut self,
        declaration: &Declaration,
    ) -> Result<Builder, CompilerError> {
        match &declaration.kind {
            DeclarationKind::Statement(statement) => match &statement.kind {
                StatementKind::Expression(expression) => self.handle_expression(expression),
                StatementKind::VariableDeclaration(declaration) => self.handle_variable_declaration(declaration),
                StatementKind::VariableAssignment(assignment) => self.handle_variable_assignment(assignment),
                _ => {
                    eprintln!("Not supported {:#?}", statement.kind);

                    todo!("Handle other statement kinds")
                }
                // StatementKind::IfStatements(if_statements) => {
                //     self.handle_if_statement(statement, if_statements)
                // }
                // StatementKind::WhileStatement(while_statement) => {
                //     self.handle_while_statement(statement, while_statement)
                // }
                // StatementKind::ReturnStatement(ret_statement) => self.handle_return_statement(statement, ret_statement),
            },
            DeclarationKind::FunctionDeclaration(fn_declaration) => {
                self.handle_function_declaration(declaration, fn_declaration)
            }
            // DeclarationKind::StructDeclaration(struct_declaration) => {
            //     self.handle_struct_declaration(declaration, struct_declaration)
            // }
            _ => {
                eprintln!("Not supported {:#?}", declaration.kind);
                todo!()
            }
        }
    }
}
