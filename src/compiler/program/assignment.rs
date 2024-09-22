use super::{
    builder::Builder,
    ir::{ExpressedType, Function, FunctionId, FunctionTemplate, Type},
    Program,
};
use crate::{
    compiler::{
        error::{CompilerError, CompilerErrorKind, WrongType},
        program::ir::{Instruction, InstructionKind, TypeKind, Variable, VariableId},
    },
    parser::{
        definition::{ExpressionKind, Primary, VariableAssignment, VariableDeclaration},
        types::{Type as ParserType, TypeKind as ParserTypeKind},
    },
};

/*
    ```rost
    fn main() {
        let a = 1 + 2;
        let b = a + 3 * 4;

        printf("%i %i", a, b);
    }
    ```

    ```irrepresentation
    global_vars = ["%i %i\n"]
    instructions = [
        Push(Int(1), Int),
        Push(Int(2), Int),
        IntAdd,
        Assign(Var(0), Int),
        Push(Int(3), Int),
        Push(Int(4), Int),
        IntMul,
        Assign(Var(1), Int),
        Push(Var(1), Int),
        Push(Var(0), Int),
        Push(GlobalVar(0), String),
        BuiltinFunction("printf", 3),
        Pop,
    ]
    ```
*/

impl Program {
    pub fn insert_type(&mut self, identifier: Option<&str>, typ: Type) -> usize {
        let type_id = self.types.len();
        self.types.push(typ);

        if let Some(identifier) = identifier {
            self.get_scope_mut()
                .type_lookup
                .insert(identifier.to_string(), type_id);
        }

        type_id
    }

    pub fn insert_function_template(
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

    pub fn insert_function(&mut self, function: Function) -> FunctionId {
        let function_id = self.functions.len();
        self.functions.push(function);

        function_id
    }

    pub fn insert_variable(&mut self, variable: Variable, identifier: Option<&str>) -> VariableId {
        let variable_id = self.variables.len();

        if let Some(identifier) = identifier {
            self.get_scope_mut()
                .variable_lookup
                .insert(identifier.to_string(), variable_id);
        }

        self.variables.push(variable);

        variable_id
    }

    pub fn get_variable(&self, identifier: &str) -> Option<VariableId> {
        self.get_scope().variable_lookup.get(identifier).copied()
    }

    pub fn get_declared_type(&self, typ: &ParserType) -> Option<ExpressedType> {
        let type_lookup = &self.get_scope().type_lookup;

        match &typ.kind {
            ParserTypeKind::Identifier(ref identifier) => {
                let type_id = type_lookup.get(identifier)?;
                let ctyp = &self.types[*type_id];

                match &ctyp.kind {
                    TypeKind::Intrinsic => Some(ExpressedType {
                        // identifier: identifier.clone(),
                        id: *type_id,
                        arguments: None,
                    }),
                    TypeKind::Function {
                        parameter_type_ids,
                        vararg_parameter_type_id,
                    } => todo!(),
                    TypeKind::UserDefined {
                        identifier,
                        declaration_pos,
                    } => todo!(),
                    _ => todo!(),
                }
            }
            ParserTypeKind::Composed {
                identifier,
                children,
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
        let picked_typ = declaration
            .typ
            .as_ref()
            .and_then(|v| self.get_declared_type(v));

        if let Some(picked_typ) = picked_typ {
            if inferred_typ != picked_typ {
                return Err(CompilerError::new(
                    declaration.right_pos.clone(),
                    CompilerErrorKind::WrongAssignmentType {
                        typ: WrongType::from_expressed_type(inferred_typ, self),
                        got: WrongType::from_expressed_type(picked_typ, self),
                        declaration_pos: declaration.typ.as_ref().map(|typ| typ.pos.clone()),
                    },
                ));
            }
        }

        let variable_id = self.insert_variable(
            Variable {
                identifier: declaration.identifier.clone(),
                typ: inferred_typ,
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
                    return Err(CompilerError::new(
                        assignment.left_pos.clone(),
                        CompilerErrorKind::UndefinedVariable(identifier.clone()),
                    ));
                };

                if infered_left != infered_right {
                    return Err(CompilerError::new(
                        assignment.right_pos.clone(),
                        CompilerErrorKind::WrongAssignmentType {
                            typ: WrongType::from_expressed_type(infered_left, self),
                            got: WrongType::from_expressed_type(infered_right, self),
                            declaration_pos: Some(
                                self.variables[variable_id].declaration_pos.clone(),
                            ),
                        },
                    ));
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

    // pub fn handle_variable_assignment(
    //     &mut self,
    //     assignment: &VariableAssignment,
    // ) -> Result<Builder, CompilerError> {
    //     let infered_left = self.infer_type(&assignment.left)?;
    //     let infered_right = self.infer_type(&assignment.right)?;

    //     let builder = match &assignment.left.kind {
    //         ExpressionKind::Primary(Primary::Identifier(identifier)) => {
    //             let Some(variable) = self.get_variable(identifier) else {
    //                 return Err(CompilerError::new(
    //                     assignment.left_pos.clone(),
    //                     CompilerErrorKind::UndefinedVariable(identifier.clone()),
    //                 ));
    //             };

    //             if infered_left != infered_right {
    //                 return Err(CompilerError::new(
    //                     assignment.right_pos.clone(),
    //                     CompilerErrorKind::WrongAssignmentType {
    //                         got: infered_right,
    //                         typ: infered_left,
    //                         declaration_pos: Some(variable.pos.clone()),
    //                     },
    //                 ));
    //             }

    //             let location = variable.location.clone();

    //             Builder::new()
    //                 .append(self.handle_expression(&assignment.right)?)
    //                 .push(Procedure {
    //                     pos: assignment.left_pos.start..assignment.right_pos.end,
    //                     comment: Some(format!("Reassign: {identifier}")),
    //                     kind: ProcedureKind::Assign(Assign {
    //                         location,
    //                         size: Self::get_type_size(&infered_right),
    //                     }),
    //                 })
    //         }
    //         ExpressionKind::Unary(Unary {
    //             expr,
    //             operator,
    //             operator_pos: _operator_pos,
    //         }) => {
    //             if *operator != Keyword::Asterix {
    //                 todo!()
    //             }

    //             if infered_left != infered_right {
    //                 return Err(CompilerError::new(
    //                     assignment.right_pos.clone(),
    //                     CompilerErrorKind::WrongAssignmentType {
    //                         got: infered_right,
    //                         typ: infered_left,
    //                         declaration_pos: None,
    //                     },
    //                 ));
    //             }

    //             Builder::new()
    //                 .push(Procedure::new(
    //                     assignment.left_pos.clone(),
    //                     ProcedureKind::Comment(format!("Assignment: {expr:?}")),
    //                 ))
    //                 .append(self.handle_expression(expr)?)
    //                 .append(self.handle_expression(&assignment.right)?)
    //                 .push(Procedure {
    //                     pos: assignment.left_pos.start..assignment.right_pos.end,
    //                     comment: Some("Reassign pointer value".to_string()),
    //                     kind: ProcedureKind::Assign(Assign {
    //                         location: VariableLocation::Address,
    //                         size: Self::get_type_size(&VariableType::Value(Keyword::Int)), // todo: maybe wrong size?
    //                     }),
    //                 })
    //         }
    //         ExpressionKind::MemberAccess(access) => {
    //             let field_type = self.get_struct_field_type(&access.left, &access.member)?;

    //             if infered_left != infered_right {
    //                 return Err(CompilerError::new(
    //                     assignment.right_pos.clone(),
    //                     CompilerErrorKind::WrongAssignmentType {
    //                         got: infered_right,
    //                         typ: infered_left,
    //                         declaration_pos: Some(field_type.pos.clone()),
    //                     },
    //                 ));
    //             }

    //             Builder::new()
    //                 .append(self.handle_member_access_without_deref(&assignment.left, access)?)
    //                 .append(self.handle_expression(&assignment.right)?)
    //                 .push(Procedure {
    //                     pos: assignment.left_pos.start..assignment.right_pos.end,
    //                     comment: Some(format!("Reassign member: {}", access.member)),
    //                     kind: ProcedureKind::Assign(Assign {
    //                         location: VariableLocation::Address,
    //                         size: Self::get_type_size(&infered_right),
    //                     }),
    //                 })
    //         }
    //         _ => {
    //             todo!("Unknown {:?}", assignment.left.kind)
    //         }
    //     };

    //     Ok(builder)
    // }
}
