use crate::{
    compiler::scope::variable::VariableType,
    lexer::Keyword,
    parser::definition::{ArrayIndex, Expression},
};

use super::{
    builder::Builder,
    definition::{Arithmetic, OperandValue, Procedure, ProcedureKind},
    error::CompilerError,
    program::Program,
};

impl Program {
    pub fn handle_array_index_without_deref(
        &mut self,
        expression: &Expression,
        index: &ArrayIndex,
    ) -> Result<Builder, CompilerError> {
        let index_type = self.infer_type(&index.index)?;
        if !matches!(index_type, VariableType::Value(Keyword::Int)) {
            todo!("Cannot index using a non integer value");
        }

        let pointer_type = self.infer_type(&index.left)?;
        let value_type = match pointer_type {
            VariableType::Pointer(typ) => typ,
            _ => todo!("Cannot index into non pointer value"),
        };

        let builder = Builder::new()
            .append(self.handle_expression(&index.left)?)
            .append(self.handle_expression(&index.index)?)
            .push(Procedure::new(
                expression.pos.clone(),
                ProcedureKind::Push(OperandValue::Int(Self::get_type_size(&value_type) as i32)),
            ))
            .push(Procedure::new(
                expression.pos.clone(),
                ProcedureKind::Arithmetic(Arithmetic::Multiply),
            ))
            .push(Procedure::new(
                expression.pos.clone(),
                ProcedureKind::Arithmetic(Arithmetic::Add),
            ));

        Ok(builder)
    }

    pub fn handle_array_index(
        &mut self,
        expression: &Expression,
        index: &ArrayIndex,
    ) -> Result<Builder, CompilerError> {
        let builder = self
            .handle_array_index_without_deref(expression, index)?
            .push(Procedure::new(expression.pos.clone(), ProcedureKind::Deref));

        Ok(builder)
    }
}
