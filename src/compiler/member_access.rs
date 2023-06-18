use crate::{
    compiler::definition::{Arithmetic, OperandValue, Procedure, ProcedureKind},
    parser::definition::{Expression, MemberAccess},
};

use super::{builder::Builder, error::CompilerError, program::Program, definition::RegisterSize};

impl Program {
    pub fn handle_member_access_without_deref(
        &mut self,
        expression: &Expression,
        access: &MemberAccess,
    ) -> Result<Builder, CompilerError> {
        let field_offset = self
            .get_struct_field_type(&access.left, &access.member)?
            .offset as i32;

        let builder = Builder::new()
            .append(self.handle_ref(&access.left)?)
            .push(Procedure::new(
                expression.pos.clone(),
                ProcedureKind::Push(OperandValue::Int(-field_offset)),
            ))
            .push(Procedure::new(
                expression.pos.clone(),
                ProcedureKind::Arithmetic(Arithmetic::Add, RegisterSize::B64),
            ));

        Ok(builder)
    }

    pub fn handle_member_access(
        &mut self,
        expression: &Expression,
        access: &MemberAccess,
    ) -> Result<Builder, CompilerError> {
        let builder = self
            .handle_member_access_without_deref(expression, access)?
            .push(Procedure::new(expression.pos.clone(), ProcedureKind::Deref));

        Ok(builder)
    }
}
