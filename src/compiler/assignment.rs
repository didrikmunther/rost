use crate::{
    lexer::Keyword,
    parser::definition::{ExpressionKind, Primary, Unary, VariableAssignment, VariableDeclaration},
};

use super::{
    builder::Builder,
    definition::{Assign, Procedure, ProcedureKind},
    error::{CompilerError, CompilerErrorKind},
    program::Program,
    scope::variable::{Variable, VariableLocation, VariableType},
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

        let size = Self::get_type_size(&infered);
        let location = self.create_variable(
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
            kind: ProcedureKind::Assign(Assign { location, size }),
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
                let Some(variable) = self.get_variable(identifier) else {
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

                let location = variable.location.clone();

                Builder::new()
                    .append(self.handle_expression(&assignment.right)?)
                    .push(Procedure {
                        pos: assignment.left_pos.start..assignment.right_pos.end,
                        comment: Some(format!("Reassign: {identifier}")),
                        kind: ProcedureKind::Assign(Assign {
                            location,
                            size: Self::get_type_size(&infered_right),
                        }),
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
                        ProcedureKind::Comment(format!("Assignment: {expr:?}")),
                    ))
                    .append(self.handle_expression(expr)?)
                    .append(self.handle_expression(&assignment.right)?)
                    .push(Procedure {
                        pos: assignment.left_pos.start..assignment.right_pos.end,
                        comment: Some("Reassign pointer value".to_string()),
                        kind: ProcedureKind::Assign(Assign {
                            location: VariableLocation::Address,
                            size: Self::get_type_size(&VariableType::Value(Keyword::Int)), // todo: maybe wrong size?
                        }),
                    })
            }
            _ => {
                todo!("Unknown {:?}", assignment.left.kind)
            }
        };

        Ok(builder)
    }
}
