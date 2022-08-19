use crate::{
    compiler::{
        builder::Builder,
        definition::{OperandValue, Procedure, ProcedureKind},
        error::{CompilerError, CompilerErrorKind},
        program::Program,
    },
    parser::definition::Expression,
};

impl Program {
    pub fn handle_identifier(
        &mut self,
        expression: &Expression,
        identifier: &String,
    ) -> Result<Builder, CompilerError> {
        if let Some(variable) = self.variables.get(identifier) {
            Ok(Builder::new().push(Procedure::new(
                expression.pos.clone(),
                ProcedureKind::Push(OperandValue::StackLocation(variable.stack_pos)),
            )))
        } else {
            return Err(CompilerError::new(
                expression.pos.clone(),
                CompilerErrorKind::UndefinedVariable(identifier.clone()),
            ));
        }
    }
}
