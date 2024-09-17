use crate::{
    compiler::{
        error::{CompilerError, CompilerErrorKind},
        program::{
            builder::Builder,
            ir::{Instruction, InstructionKind, ValueKind},
            Program,
        },
    },
    parser::definition::Expression,
};

impl Program {
    /// Pushes the value of the identifier to the stack.
    pub fn handle_identifier(
        &mut self,
        expression: &Expression,
        identifier: &str,
        _load_address: bool,
    ) -> Result<Builder, CompilerError> {
        let Some(variable_id) = self.get_variable(identifier) else {
            return Err(CompilerError::new(
                expression.pos.clone(),
                CompilerErrorKind::UndefinedVariable(identifier.to_string()),
            ));
        };

        Ok(Builder::new().push(Instruction::new(
            expression.pos.clone(),
            InstructionKind::Push(ValueKind::Variable(variable_id)),
        )))
    }
}
