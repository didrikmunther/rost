use crate::{
    compiler::{
        builder::Builder,
        definition::{OperandValue, Procedure, ProcedureKind},
        error::{CompilerError, CompilerErrorKind},
        program::Program,
        scope::variable::{VariableLocation, VariableType},
    },
    lexer::Keyword,
    parser::definition::Expression,
};

impl Program {
    pub fn handle_identifier(
        &mut self,
        expression: &Expression,
        identifier: &String,
    ) -> Result<Builder, CompilerError> {
        if let Some(variable) = self.get_variable(identifier) {
            let is_pointer = match &variable.typ {
                VariableType::Value(typ) => *typ == Keyword::String,
                _ => false,
            };

            let operand_value = match &variable.location {
                VariableLocation::Stack(loc) => OperandValue::StackLocation(*loc),
                VariableLocation::Global(label) => {
                    if is_pointer {
                        OperandValue::DataPointerLocation(label.clone())
                    } else {
                        OperandValue::DataLocation(label.clone())
                    }
                }
            };

            Ok(Builder::new().push(Procedure::new(
                expression.pos.clone(),
                ProcedureKind::Push(operand_value),
            )))
        } else {
            return Err(CompilerError::new(
                expression.pos.clone(),
                CompilerErrorKind::UndefinedVariable(identifier.clone()),
            ));
        }
    }
}
