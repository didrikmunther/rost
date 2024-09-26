use super::{
    builder::Builder,
    ir::{FunctionTemplate, Variable},
    typ::{ExpressedType, FunctionTypeKind, Type, TypeIdWithIdentifier, TypeKind},
    Program,
};
use crate::{
    compiler::error::CompilerError,
    parser::definition::{Declaration, FunctionDeclaration},
};

impl Program {
    fn insert_function_template(
        &mut self,
        identifier: &str,
        function_template: FunctionTemplate,
    ) -> usize {
        let function_template_id = self.function_templates.len();
        self.function_templates.push(function_template);

        self.get_scope_mut()
            .function_template_lookup
            .insert(identifier.to_string(), function_template_id);

        function_template_id
    }

    pub fn create_function_declaration(
        &mut self,
        fdec: &FunctionDeclaration,
    ) -> Result<(), CompilerError> {
        let mut parameter_type_ids = vec![];

        if let Some(type_params) = &fdec.type_params {
            for type_param in type_params {
                self.insert_type(
                    Some(&type_param.identifier),
                    Type {
                        kind: TypeKind::Generic {
                            identifier: type_param.identifier.clone(),
                            declaration_pos: type_param.pos.clone(),
                        },
                        type_parameters: vec![],
                    },
                );
            }
        }

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

        self.insert_variable(
            Variable {
                typ: ExpressedType {
                    id: function_type_id,
                    arguments: None,
                },
                declaration_pos: fdec.identifier_pos.clone(),
                assignment_has_error: false,
                identifier: fdec.identifier.clone(),
            },
            Some(&fdec.identifier),
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
