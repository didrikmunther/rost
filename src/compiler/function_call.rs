use crate::parser::definition::{Expression, FunctionCall};

use super::{
    builder::Builder,
    definition::{Procedure, ProcedureCall, ProcedureKind},
    error::CompilerError,
    program::Program,
};

impl Program {
    pub fn handle_function_call(
        &mut self,
        expression: &Expression,
        fcall: &FunctionCall,
    ) -> Result<Builder, CompilerError> {
        let mut builder = Builder::new();

        for arg in &fcall.args {
            let expr = self.handle_expression(arg)?;
            builder = builder.append(expr);
        }

        builder = builder.push(Procedure::new(
            expression.pos.clone(),
            ProcedureKind::ProcedureCall(ProcedureCall {
                nargs: fcall.args.len(),
                identifier: fcall.identifier.clone(),
            }),
        ));

        Ok(builder)
    }
}
