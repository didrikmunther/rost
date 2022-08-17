use crate::parser::definition::{Expression, ExpressionKind};

use super::{
    definition::{Procedure, ProcedureKind},
    error::{CompilerError, CompilerErrorKind},
    program::Program,
};

impl Program {
    pub fn handle_expression(
        &mut self,
        expression: &Expression,
    ) -> Result<Procedure, CompilerError> {
        match &expression.kind {
            ExpressionKind::FunctionCall(fcall) => Ok(Procedure {
                pos: expression.pos.clone(),
                kind: ProcedureKind::SystemCall(self.handle_function_call(fcall)?),
            }),
            _ => {
                return Err(CompilerError::new(
                    expression.pos.clone(),
                    CompilerErrorKind::Unimplemented,
                ))
            }
        }
    }
}
