use crate::{
    lexer::Keyword,
    parser::definition::{ExpressionKind, Primary, Unary, VariableAssignment, VariableDeclaration},
};

use super::{
    builder::Builder,
    definition::{Procedure, ProcedureKind},
    error::{CompilerError, CompilerErrorKind},
    program::Program,
    scope::variable::{Variable, VariableLocation},
};

impl Program {
    pub fn handle_variable_declaration(
        &mut self,
        declaration: &VariableDeclaration,
    ) -> Result<Builder, CompilerError> {
        if let Some(variable) = self.get_variable(&declaration.identifier) {
            return Err(CompilerError::new(
                declaration.identifier_pos.clone(),
                CompilerErrorKind::RedeclaredVariable(
                    declaration.identifier.clone(),
                    variable.pos.clone(),
                ),
            ));
        }

        let infered = self.infer_type(&declaration.right)?;
        let typ = declaration
            .typ
            .as_ref()
            .map(|typ| self.get_variable_type(typ));

        if let Some(typ) = typ {
            if typ != infered {
                return Err(CompilerError::new(
                    declaration.right_pos.clone(),
                    CompilerErrorKind::WrongType {
                        got: infered,
                        expected: typ,
                    },
                ));
            }
        }

        let variable_location = self.create_variable(
            declaration.identifier.clone(),
            Variable {
                pos: declaration.identifier_pos.clone(),
                typ: infered,
            },
        );

        let value = self.handle_expression(&declaration.right)?;

        let builder = Builder::new()
            .push(Procedure::new(
                declaration.identifier_pos.clone(),
                ProcedureKind::Comment(format!(
                    "Variable declaration: {}",
                    declaration.identifier.clone()
                )),
            ))
            .append(value);

        return Ok(builder.push(Procedure {
            pos: declaration.identifier_pos.start..declaration.right_pos.end,
            comment: Some(format!("Assign: {}", declaration.identifier)),
            kind: ProcedureKind::Assign(variable_location.clone()),
        }));
    }

    pub fn handle_variable_assignment(
        &mut self,
        assignment: &VariableAssignment,
    ) -> Result<Builder, CompilerError> {
        let infered_left = self.infer_type(&assignment.left)?;
        let infered_right = self.infer_type(&assignment.right)?;

        let builder = match &assignment.left.kind {
            ExpressionKind::Primary(Primary::Identifier(identifier)) => {
                let Some(variable) = self.get_variable(&identifier) else {
                    return Err(CompilerError::new(
                        assignment.left_pos.clone(),
                        CompilerErrorKind::UndefinedVariable(identifier.clone()),
                    ));
                };

                if infered_left != infered_right {
                    return Err(CompilerError::new(
                        assignment.right_pos.clone(),
                        CompilerErrorKind::WrongAssignmentType {
                            got: infered_right,
                            typ: infered_left,
                            declaration_pos: Some(variable.pos.clone()),
                        },
                    ));
                }

                let var_location = variable.location.clone();

                Builder::new()
                    .append(self.handle_expression(&assignment.right)?)
                    .push(Procedure {
                        pos: assignment.left_pos.start..assignment.right_pos.end,
                        comment: Some(format!("Reassign: {}", identifier)),
                        kind: ProcedureKind::Assign(var_location),
                    })
            }
            ExpressionKind::Unary(Unary {
                expr,
                operator,
                operator_pos: _operator_pos,
            }) => {
                if *operator != Keyword::Asterix {
                    todo!()
                }

                if infered_left != infered_right {
                    return Err(CompilerError::new(
                        assignment.right_pos.clone(),
                        CompilerErrorKind::WrongAssignmentType {
                            got: infered_right,
                            typ: infered_left,
                            declaration_pos: None,
                        },
                    ));
                }

                Builder::new()
                    .push(Procedure::new(
                        assignment.left_pos.clone(),
                        ProcedureKind::Comment(format!("Assignment: {:?}", expr)),
                    ))
                    .append(self.handle_expression(&expr)?)
                    .append(self.handle_expression(&assignment.right)?)
                    .push(Procedure {
                        pos: assignment.left_pos.start..assignment.right_pos.end,
                        comment: Some(format!("Reassign pointer value")),
                        kind: ProcedureKind::Assign(VariableLocation::Address),
                    })
            }
            _ => {
                todo!("Unknown {:?}", assignment.left.kind)
            }
        };

        return Ok(builder);

        // if let Some(variable) = self.get_variable(&assignment.identifier) {
        //     if assignment.is_new {
        //         return Err(CompilerError::new(
        //             assignment.identifier_pos.clone(),
        //             CompilerErrorKind::RedeclaredVariable(
        //                 assignment.identifier.clone(),
        //                 variable.pos.clone(),
        //             ),
        //         ));
        //     } else {
        //         let assignment_type = self.infer_type(&assignment.value)?;
        //         if assignment_type != variable.typ {
        //             return Err(CompilerError::new(
        //                 assignment.value_pos.clone(),
        //                 CompilerErrorKind::WrongAssignmentType {
        //                     got: assignment_type,
        //                     typ: variable.typ.clone(),
        //                     declaration_pos: variable.pos.clone(),
        //                 },
        //             ));
        //         }

        //         return Ok(builder.push(Procedure {
        //             pos: assignment.identifier_pos.start..assignment.value_pos.end,
        //             comment: Some(format!("Reassign: {}", assignment.identifier)),
        //             kind: ProcedureKind::Assign(variable.location.clone()),
        //         }));
        //     }
        // } else if !assignment.is_new {
        //     return Err(CompilerError::new(
        //         assignment.identifier_pos.clone(),
        //         CompilerErrorKind::UndefinedVariable(assignment.identifier.clone()),
        //     ));
        // }

        // if assignment.is_new {
        //     let infered = self.infer_type(&assignment.value)?;
        //     if let Some(typ) = assignment
        //         .typ
        //         .as_ref()
        //         .map(|typ| self.get_variable_type(&typ))
        //     {
        //         if typ != infered {
        //             return Err(CompilerError::new(
        //                 assignment.value_pos.clone(),
        //                 CompilerErrorKind::WrongType {
        //                     got: infered,
        //                     expected: typ,
        //                 },
        //             ));
        //         }
        //     }

        //     let variable_location = self.create_variable(
        //         assignment.identifier.clone(),
        //         Variable {
        //             pos: assignment.identifier_pos.clone(),
        //             typ: infered,
        //         },
        //     );

        //     return Ok(builder.push(Procedure {
        //         pos: assignment.identifier_pos.start..assignment.value_pos.end,
        //         comment: Some(format!("Assign: {}", assignment.identifier)),
        //         kind: ProcedureKind::Assign(variable_location.clone()),
        //     }));
        // }
    }
}
