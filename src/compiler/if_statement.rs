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
        if_statement: &IfStatement,
    ) -> Result<Builder, CompilerError> {
        match self.infer_type(&if_statement.condition)? {
            Keyword::Bool => {}
            _ => todo!("error"),
        };

        let condition = self.handle_expression(&if_statement.condition)?;
        let mut builder = Builder::new().append(condition);
        let mut content = Builder::new();

        for declaration in &if_statement.content {
            content = content.append(self.handle_declaration(declaration)?);
        }

        builder = builder.push(Procedure::new(
            statement.pos.clone(),
            ProcedureKind::If(If {
                content: Box::new(content),
            }),
        ));

        Ok(builder)
    }
}
