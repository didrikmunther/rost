use crate::{
    lexer::Keyword,
    parser::definition::{Expression, FunctionCall},
};

use super::{
    builder::Builder,
    definition::{Procedure, ProcedureCall, ProcedureKind, SystemCall},
    error::{CompilerError, CompilerErrorKind},
    program::Program,
    scope::VariableType,
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

        if BUILT_IN.contains(&fcall.identifier.as_str()) {
            return Ok(builder.push(Procedure::new(
                expression.pos.clone(),
                ProcedureKind::SystemCall(SystemCall {
                    nargs: fcall.args.len(),
                    identifier: fcall.identifier.clone(),
                }),
            )));
        }

        let Some(variable) = self.get_variable(&fcall.identifier) else {
            return Err(CompilerError::new(
                fcall.identifier_pos.clone(),
                CompilerErrorKind::UndefinedFunction(fcall.identifier.clone()),
            ));
        };

        if variable.typ.to_keyword() != Keyword::Function {
            todo!("Non-function variable already declared with same identifier")
        }

        let VariableType::Function(function_id) = variable.typ else {
            todo!("Variable is not a function")
        };

        let function = self.functions.get(function_id).unwrap();

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

        Ok(builder)
    }
}
