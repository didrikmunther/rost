use crate::{
    compiler::{
        builder::Builder,
        definition::{Procedure, ProcedureKind},
        error::CompilerError,
        program::Program,
    },
    parser::definition::{Expression, ExpressionKind, Primary},
};

impl Program {
    pub fn handle_ref(&mut self, expression: &Expression) -> Result<Builder, CompilerError> {
        match &expression.kind {
            ExpressionKind::Primary(primary) => match &primary {
                Primary::Identifier(identifier) => Ok(
                    Builder::new().append(self.handle_identifier(expression, identifier, true)?)
                ),
                _ => todo!("Not supported"),
            },
            ExpressionKind::MemberAccess(access) => {
                self.handle_member_access_without_deref(expression, access)
            }
            _ => {
                todo!("Not supported {:?}", expression.kind);
            }
        }
    }

    pub fn handle_deref(
        &mut self,
        complete_expression: &Expression,
        expression: &Expression,
    ) -> Result<Builder, CompilerError> {
        Ok(Builder::new()
            .append(self.handle_expression(expression)?)
            .push(Procedure::new(
                complete_expression.pos.clone(),
                ProcedureKind::Deref,
            )))
    }
}
