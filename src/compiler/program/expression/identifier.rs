use crate::{
    compiler::{
        error::CompilerError,
        program::{builder::Builder, Program},
    },
    parser::definition::Expression,
};

impl Program {
    /// Pushes the value of the identifier to the stack.
    /// If `load_address`, uses `LEA` instead of `MOV`
    pub fn handle_identifier(
        &mut self,
        _expression: &Expression,
        _identifier: &String,
        _load_address: bool,
    ) -> Result<Builder, CompilerError> {
        todo!()

        // if let Some(variable) = self.get_variable(identifier) {
        //     let operand_value = match &variable.location {
        //         VariableLocation::Stack(loc) => OperandValue::StackLocation(*loc),
        //         VariableLocation::Global(label) => {
        //             let is_pointer = matches!(&variable.typ, VariableType::Pointer(_));

        //             if is_pointer {
        //                 OperandValue::DataPointerLocation(label.clone())
        //             } else {
        //                 OperandValue::DataLocation(label.clone())
        //             }
        //         }
        //         VariableLocation::Address => {
        //             todo!()
        //         }
        //     };

        //     let operation = if load_address {
        //         ProcedureKind::PushAddress(operand_value)
        //     } else {
        //         ProcedureKind::Push(operand_value)
        //     };

        //     Ok(Builder::default().push(Procedure::new(expression.pos.clone(), operation)))
        // } else {
        //     Err(CompilerError::new(
        //         expression.pos.clone(),
        //         CompilerErrorKind::UndefinedVariable(identifier.clone()),
        //     ))
        // }
    }
}
