use crate::{
    compiler::{
        builder::Builder,
        definition::{Arithmetic, Procedure, ProcedureKind},
        error::{CompilerError, CompilerErrorKind},
        program::Program,
    },
    parser::definition::{Binary, Expression, ExpressionKind},
};

impl Program {
    pub fn handle_binary(
        &mut self,
        expression: &Expression,
        binary: &Binary,
    ) -> Result<Builder, CompilerError> {
        let builder = Builder::new()
            .append(self.handle_expression(&binary.left)?)
            .append(self.handle_expression(&binary.right)?)
            .push(Procedure::new(
                expression.pos.clone(),
                ProcedureKind::Arithmetic(Arithmetic::Add),
            ));

        Ok(builder)
    }

    pub fn handle_expression(&mut self, expression: &Expression) -> Result<Builder, CompilerError> {
        match &expression.kind {
            ExpressionKind::FunctionCall(fcall) => self.handle_function_call(expression, fcall),
            ExpressionKind::Primary(primary) => self.handle_primary(expression, primary),
            ExpressionKind::Binary(binary) => self.handle_binary(expression, binary),
            _ => {
                return Err(CompilerError::new(
                    expression.pos.clone(),
                    CompilerErrorKind::Unimplemented(format!("{:?}", expression.kind)),
                ))
            }
        }
    }
}
