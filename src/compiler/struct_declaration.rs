use std::collections::HashMap;

use crate::parser::definition::{Declaration, StructDeclaration};

use super::{
    builder::Builder,
    definition::{Struct, StructField},
    error::CompilerError,
    program::Program,
    scope::{
        variable::{StructType, Variable, VariableType},
        ProgramScope,
    },
};

impl Program {
    pub fn handle_struct_declaration(
        &mut self,
        statement: &Declaration,
        sdec: &StructDeclaration,
    ) -> Result<Builder, CompilerError> {
        let ProgramScope::RootScope(_) = &mut self.scope else {
            todo!("Must declare struct in a root scope");
        };

        let mut fields = HashMap::new();
        let mut offset = 0;

        for (identifier, field) in &sdec.fields {
            let typ = self.get_variable_type(&field.typ);
            let size = Self::get_type_size(&typ);

            fields.insert(
                identifier.clone(),
                StructField {
                    typ,
                    offset,
                    size,
                    pos: field.pos.clone(),
                },
            );

            offset += size;
        }

        let struct_size = fields.values().map(|field| field.size).sum();

        self.structs.push(Struct {
            fields,
            size: offset,
        });

        let struct_id = self.structs.len() - 1;

        // Pseudo-type-ish variable, does not exist on the stack.
        self.create_variable(
            sdec.identifier.clone(),
            Variable {
                pos: statement.pos.clone(),
                typ: VariableType::Struct(StructType {
                    id: struct_id,
                    size: struct_size,
                }),
            },
        );

        Ok(Builder::new())
    }
}
