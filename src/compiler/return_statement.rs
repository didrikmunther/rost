use crate::parser::definition::{ReturnStatement, Statement};

use super::{
    builder::Builder,
    definition::{Procedure, ProcedureKind},
    error::CompilerError,
    program::Program,
};

impl Program {
    pub fn handle_return_statement(
        &mut self,
        statement: &Statement,
        ret_statement: &ReturnStatement,
    ) -> Result<Builder, CompilerError> {
        let typ = self.infer_type(&ret_statement.value)?;
        let keyword = self.get_return_type().get_keyword_type();

        if keyword != typ {
            todo!("Wrong return type")
        }

        let builder = Builder::new()
            .append(self.handle_expression(&ret_statement.value)?)
            .push(Procedure::new(statement.pos.clone(), ProcedureKind::Return));

        Ok(builder)
    }
}
