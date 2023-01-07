use crate::{
    compiler::definition::If,
    lexer::Keyword,
    parser::definition::{IfStatement, Statement},
};

use super::{
    builder::Builder,
    definition::{Procedure, ProcedureKind},
    error::CompilerError,
    program::Program,
};

impl Program {
    pub fn handle_if_statement(
        &mut self,
        statement: &Statement,
        if_statements: &Vec<IfStatement>,
    ) -> Result<Builder, CompilerError> {
        let mut ifs = Vec::new();

        for if_statement in if_statements {
            let condition = if_statement
                .condition
                .as_ref()
                .map_or(Ok(None), |condition| match self.infer_type(&condition)? {
                    Keyword::Bool => Ok(Some(self.handle_expression(&condition)?)),
                    _ => todo!("error"),
                })?
                .map(Box::new);

            let content = self.with_scope(|this| this.get_procedures(&if_statement.content))?;

            ifs.push(If {
                condition,
                content: Box::new(content),
            });
        }

        let builder = Builder::new().push(Procedure::new(
            statement.pos.clone(),
            ProcedureKind::If(ifs),
        ));

        Ok(builder)
    }
}
