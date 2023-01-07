use crate::{
    lexer::Keyword,
    parser::definition::{Expression, FunctionCall},
};

use super::{
    builder::Builder,
    definition::{Procedure, ProcedureCall, ProcedureKind, SystemCall},
    error::{CompilerError, CompilerErrorKind},
    program::Program,
};

static BUILT_IN: &[&'static str] = &["printf"];

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

        if let Some(variable) = self.get_variable(&fcall.identifier) {
            if variable.typ != Keyword::Function {
                unimplemented!() // Non-function variable already declared with same identifier
            }

            let function_id = variable.stack_pos; // stack_pos is used as an index to the function vector (temporary hack)
            let function = self.functions.get(variable.stack_pos).unwrap();

            if function.npars != fcall.args.len() {
                todo!(
                    "Wrong number of arguments to function, takes {}, {} was given",
                    function.npars,
                    fcall.args.len()
                )
            }

            builder = builder.push(Procedure::new(
                expression.pos.clone(),
                ProcedureKind::ProcedureCall(ProcedureCall {
                    function_id,
                    nargs: fcall.args.len(),
                    returns: function.return_type.returns(),
                }),
            ));
        } else if BUILT_IN.contains(&fcall.identifier.as_str()) {
            builder = builder.push(Procedure::new(
                expression.pos.clone(),
                ProcedureKind::SystemCall(SystemCall {
                    nargs: fcall.args.len(),
                    identifier: fcall.identifier.clone(),
                }),
            ));
        } else {
            return Err(CompilerError::new(
                fcall.identifier_pos.clone(),
                CompilerErrorKind::UndefinedFunction(fcall.identifier.clone()),
            ));
        }

        Ok(builder)
    }
}
