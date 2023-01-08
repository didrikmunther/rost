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
    pub fn handle_ref(
        &mut self,
        _complete_expression: &Expression,
        expression: &Expression,
    ) -> Result<Builder, CompilerError> {
        match &expression.kind {
            ExpressionKind::Primary(primary) => match &primary {
                Primary::Identifier(identifier) => {
                    return Ok(Builder::new().append(self.handle_identifier(
                        expression,
                        &identifier,
                        true,
                    )?));
                }
                _ => todo!("Not supported"),
            },
            _ => todo!("Not supported"),
        }
    }

    pub fn handle_deref(
        &mut self,
        complete_expression: &Expression,
        expression: &Expression,
    ) -> Result<Builder, CompilerError> {
        let typ = self.infer_type(expression)?;
        println!("typ: {typ:?}");

        Ok(Builder::new()
            .append(self.handle_expression(expression)?)
            .push(Procedure::new(
                complete_expression.pos.clone(),
                ProcedureKind::Deref,
            )))
    }
}
