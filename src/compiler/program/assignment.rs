use super::{
    builder::Builder,
    typ::{ExpressedType, FunctionTypeKind, TypeKind},
    Program,
};
use crate::{
    compiler::{
        error::{CompilerError, CompilerErrorKind, WrongType},
        program::ir::{Instruction, InstructionKind, Variable},
    },
    parser::{
        definition::{ExpressionKind, Primary, VariableAssignment, VariableDeclaration},
        types::{Type as ParserType, TypeKind as ParserTypeKind},
    },
};

impl Program {
    pub fn get_declared_type(&self, typ: &ParserType) -> Option<ExpressedType> {
        let type_lookup = &self.get_scope().type_lookup;

        match &typ.kind {
            ParserTypeKind::Identifier(ref identifier) => {
                let type_id = type_lookup.get(identifier)?;
                let ctyp = &self.types[*type_id];

                match &ctyp.kind {
                    TypeKind::Intrinsic { identifier: _ } => Some(ExpressedType {
                        // identifier: identifier.clone(),
                        id: *type_id,
                        arguments: None,
                    }),
                    TypeKind::Function(FunctionTypeKind {
                        parameter_type_ids: _,
                        vararg_parameter_type_id: _,
                        declaration_pos: _,
                    }) => todo!(),
                    TypeKind::UserDefined {
                        identifier: _,
                        declaration_pos: _,
                    } => todo!(),
                    TypeKind::Generic { .. } => Some(ExpressedType {
                        id: *type_id,
                        arguments: None,
                    }),
                }
            }
            ParserTypeKind::Composed {
                identifier: _,
                children: _,
            } => todo!(),
            ParserTypeKind::Pointer(_) => todo!(),
        }
    }

    pub fn handle_variable_declaration(
        &mut self,
        declaration: &VariableDeclaration,
    ) -> Result<Builder, CompilerError> {
        let result = self.handle_expression(&declaration.right);
        let value = self.get_or_add_error(result);
        let assignment_has_error = value.is_none();

        let inferred_typ = self.infer_type(&declaration.right)?;

        let assignment_typ = if let Some(declaration_typ) = &declaration.typ {
            match self.get_declared_type(declaration_typ) {
                None => {
                    self.add_error(CompilerError::new(
                        declaration_typ.pos.clone(),
                        CompilerErrorKind::UndefinedType(WrongType {
                            content: format!("{declaration_typ}"),
                        }),
                    ));

                    self.get_intrinstic_type("unknown")
                }
                Some(picked_typ) => {
                    let is_unknown = match self.reverse_type_lookup.get(&inferred_typ.id) {
                        Some(identifier) => identifier == "unknown",
                        None => false,
                    };

                    if !is_unknown && inferred_typ != picked_typ {
                        self.add_error(CompilerError::new(
                            declaration.right_pos.clone(),
                            CompilerErrorKind::WrongAssignmentType {
                                got: WrongType::from_expressed_type(inferred_typ.clone(), self),
                                typ: WrongType::from_expressed_type(picked_typ, self),
                                declaration_pos: declaration
                                    .typ
                                    .as_ref()
                                    .map(|typ| typ.pos.clone()),
                            },
                        ));

                        self.get_intrinstic_type("unknown")
                    } else {
                        inferred_typ
                    }
                }
            }
        } else {
            inferred_typ
        };

        let variable_id = self.insert_variable(
            Variable {
                identifier: declaration.identifier.clone(),
                typ: assignment_typ,
                declaration_pos: declaration.identifier_pos.clone(),
                assignment_has_error,
            },
            Some(&declaration.identifier),
        );

        let Some(value) = value else {
            return Ok(Builder::new());
        };

        let builder = Builder::new().append(value);

        Ok(builder.push(Instruction {
            pos: declaration.identifier_pos.start..declaration.right_pos.end,
            comment: Some(format!("Assign: {}", declaration.identifier)),
            kind: InstructionKind::Assign(variable_id),
        }))
    }

    pub fn handle_variable_assignment(
        &mut self,
        assignment: &VariableAssignment,
    ) -> Result<Builder, CompilerError> {
        let value = self.handle_expression(&assignment.right)?;
        let infered_left = self.infer_type(&assignment.left)?;
        let infered_right = self.infer_type(&assignment.right)?;

        let builder = match &assignment.left.kind {
            ExpressionKind::Primary(Primary::Identifier(identifier)) => {
                let Some(variable_id) = self.get_variable(identifier) else {
                    return CompilerErrorKind::UndefinedVariable(identifier.clone())
                        .at_pos(&assignment.left_pos)
                        .into();
                };

                let variable_id = variable_id.get_id();

                if infered_left != infered_right {
                    return (CompilerErrorKind::WrongAssignmentType {
                        typ: WrongType::from_expressed_type(infered_left, self),
                        got: WrongType::from_expressed_type(infered_right, self),
                        declaration_pos: Some(self.variables[variable_id].declaration_pos.clone()),
                    })
                    .at_pos(&assignment.right_pos)
                    .into();
                }

                Builder::new().append(value).push(Instruction {
                    pos: assignment.left_pos.start..assignment.right_pos.end,
                    comment: Some(format!("Assign: {}", identifier)),
                    kind: InstructionKind::Assign(variable_id),
                })
            }
            _ => todo!(),
        };

        Ok(builder)
    }
}
