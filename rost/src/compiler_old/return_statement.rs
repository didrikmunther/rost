use crate::parser::definition::{ReturnStatement, Statement};

use super::{
    builder::Builder,
    definition::{Procedure, ProcedureKind},
    error::CompilerError,
    program::Program,
    scope::ProgramScope,
};

impl Program {
    pub fn handle_return_statement(
        &mut self,
        statement: &Statement,
        ret_statement: &ReturnStatement,
    ) -> Result<Builder, CompilerError> {
        let ProgramScope::FunctionScope(function_scope) = &self.scope else {
            todo!("We're not in a function");
        };

        let Some(return_type) = &function_scope.return_type else {
            todo!("Cannot return a value from this function");
        };

        let typ = self.infer_type(&ret_statement.value)?;

        if *return_type != typ {
            todo!("Wrong return type")
        }

        let builder = Builder::new()
            .append(self.handle_expression(&ret_statement.value)?)
            .push(Procedure::new(statement.pos.clone(), ProcedureKind::Return));

        Ok(builder)
    }
}
