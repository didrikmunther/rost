use crate::{
    compiler::{
        builder::Builder,
        definition::{OperandValue, Procedure, ProcedureKind},
        error::{CompilerError, CompilerErrorKind},
        program::Program,
        scope::variable::{VariableLocation, VariableType},
    },
    parser::definition::Expression,
};

impl Program {
    /// Pushes the value of the identifier to the stack.
    /// If `load_address`, uses `LEA` instead of `MOV`
    pub fn handle_identifier(
        &mut self,
        expression: &Expression,
        identifier: &String,
        load_address: bool,
    ) -> Result<Builder, CompilerError> {
        if let Some(variable) = self.get_variable(identifier) {
            let operand_value = match &variable.location {
                VariableLocation::Stack(loc) => OperandValue::StackLocation(*loc),
                VariableLocation::Global(label) => {
                    let is_pointer = match &variable.typ {
                        VariableType::Pointer(_) => true,
                        _ => false,
                    };

                    if is_pointer {
                        OperandValue::DataPointerLocation(label.clone())
                    } else {
                        OperandValue::DataLocation(label.clone())
                    }
                }
                VariableLocation::Address => {
                    todo!()
                }
            };

            let operation = if load_address {
                ProcedureKind::PushAddress(operand_value)
            } else {
                ProcedureKind::Push(operand_value)
            };

            Ok(Builder::new().push(Procedure::new(expression.pos.clone(), operation)))
        } else {
            return Err(CompilerError::new(
                expression.pos.clone(),
                CompilerErrorKind::UndefinedVariable(identifier.clone()),
            ));
        }
    }
}
