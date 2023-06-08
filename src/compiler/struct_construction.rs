use crate::{
    compiler::scope::variable::StructType,
    parser::definition::{Expression, StructConstruction},
};

use super::{
    builder::Builder,
    error::{CompilerError, CompilerErrorKind},
    program::Program,
    scope::variable::VariableType,
};

impl Program {
    pub fn handle_struct_construction(
        &mut self,
        _expression: &Expression,
        sconst: &StructConstruction,
    ) -> Result<Builder, CompilerError> {
        let mut builder = Builder::new();

        let Some(variable) = self.get_variable(&sconst.identifier) else {
            return Err(CompilerError::new(
                sconst.identifier_pos.clone(),
                CompilerErrorKind::UndefinedFunction(sconst.identifier.clone()),
            ));
        };

        let VariableType::Struct(StructType {id, size: _size, ..}) = variable.typ else {
            todo!("Variable is not a struct")
        };

        let sdec = self.structs.get(id).unwrap();
        let mut field_content = Vec::with_capacity(sdec.fields.len());

        for (field_identifier, field_dec) in &sdec.fields {
            let Some(field_const) = sconst.fields.get(field_identifier) else {
                todo!("Missing field: {}", field_identifier);
            };

            if field_dec.typ != self.infer_type(&field_const.expr)? {
                todo!("Field wrong type");
            }

            field_content.push((field_identifier.clone(), field_const));
        }

        for (_field_identifier, field_const) in field_content {
            builder = builder.append(self.handle_expression(&field_const.expr)?);
        }

        Ok(builder)
    }
}
