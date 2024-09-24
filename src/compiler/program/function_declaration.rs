use super::{
    builder::Builder,
    ir::{FunctionTemplate, FunctionTypeKind, Type, TypeIdWithIdentifier, TypeKind},
    Program,
};
use crate::{
    compiler::error::CompilerError,
    parser::definition::{Declaration, FunctionDeclaration},
};

impl Program {
    pub fn create_function_declaration(
        &mut self,
        fdec: &FunctionDeclaration,
    ) -> Result<(), CompilerError> {
        let mut parameter_type_ids = vec![];

        for param in &fdec.parameters {
            let picked_typ = self.get_declared_type(&param.typ).unwrap();
            parameter_type_ids.push(TypeIdWithIdentifier {
                identifier: param.identifier.clone(),
                id: picked_typ.id,
            });
        }

        let vararg_parameter_type_id = fdec.vararg_parameter.as_ref().map(|vararg| {
            let picked_typ = self.get_declared_type(&vararg.typ).unwrap();
            TypeIdWithIdentifier {
                identifier: vararg.identifier.clone(),
                id: picked_typ.id,
            }
        });

        let function_type_id = self.insert_type(
            None,
            Type {
                kind: TypeKind::Function(FunctionTypeKind {
                    parameter_type_ids,
                    vararg_parameter_type_id,
                    declaration_pos: fdec.identifier_pos.clone(),
                }),
                type_parameters: vec![],
            },
        );

        self.insert_function_template(
            &fdec.identifier,
            FunctionTemplate {
                typ: function_type_id,
                function_declaration: fdec.clone(),
            },
        );

        Ok(())
    }

    pub fn handle_function_declaration(
        &mut self,
        _statement: &Declaration,
        fdec: &FunctionDeclaration,
    ) -> Result<Builder, CompilerError> {
        self.create_function_declaration(fdec)?;

        Ok(Builder::new())
    }
}
