use crate::{
    compiler::{
        builder::Builder,
        error::{CompilerError, CompilerErrorKind},
        program::Program,
    },
    parser::definition::{Expression, ExpressionKind},
};

impl Program {
    pub fn handle_expression(&mut self, expression: &Expression) -> Result<Builder, CompilerError> {
        match &expression.kind {
            ExpressionKind::FunctionCall(fcall) => self.handle_function_call(expression, fcall),
            ExpressionKind::Primary(primary) => self.handle_primary(expression, primary),
            _ => {
                return Err(CompilerError::new(
                    expression.pos.clone(),
                    CompilerErrorKind::Unimplemented(format!("{:?}", expression.kind)),
                ))
            }
        }
    }
}
