use crate::{
    lexer::Keyword,
    parser::definition::{Statement, WhileStatement},
};

use super::{
    builder::Builder,
    definition::{Procedure, ProcedureKind, While},
    error::CompilerError,
    program::Program,
};

impl Program {
    pub fn handle_while_statement(
        &mut self,
        statement: &Statement,
        while_statement: &WhileStatement,
    ) -> Result<Builder, CompilerError> {
        let condition = match self.infer_type(&while_statement.condition)? {
            Keyword::Bool => self.handle_expression(&while_statement.condition)?,
            _ => todo!("error"),
        };

        let content = self.with_scope(|this| this.get_procedures(&while_statement.content))?;

        let builder = Builder::new().push(Procedure::new(
            statement.pos.clone(),
            ProcedureKind::While(While {
                condition: Box::new(condition),
                content: Box::new(content),
            }),
        ));

        Ok(builder)
    }
}
