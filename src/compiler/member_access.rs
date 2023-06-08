use crate::{
    compiler::{
        definition::{Arithmetic, OperandValue, Procedure, ProcedureKind},
        scope::variable::VariableType,
    },
    parser::definition::{Expression, MemberAccess},
};

use super::{builder::Builder, error::CompilerError, program::Program};

impl Program {
    pub fn handle_member_access(
        &mut self,
        expression: &Expression,
        access: &MemberAccess,
    ) -> Result<Builder, CompilerError> {
        let mut builder = Builder::new();

        let VariableType::Struct(struct_type) = self.infer_type(&access.left)? else {
            todo!("Struct does not exist");
        };

        let Some(struct_declaration) = self.structs.get(struct_type.id) else {
            todo!("Struct was not found");
        };

        let Some(field_offset) = struct_declaration.fields.get(&access.member).map(|v| v.offset as i32) else {
            todo!("Field does not exist");
        };

        builder = builder
            .append(self.handle_ref(&access.left)?)
            .push(Procedure::new(
                expression.pos.clone(),
                ProcedureKind::Push(OperandValue::Int(-field_offset)),
            ))
            .push(Procedure::new(
                expression.pos.clone(),
                ProcedureKind::Arithmetic(Arithmetic::Add),
            ))
            .push(Procedure::new(expression.pos.clone(), ProcedureKind::Deref));

        Ok(builder)
    }
}
