use crate::parser::definition::Assignment;

use super::{
    builder::Builder,
    definition::{Procedure, ProcedureKind},
    error::{CompilerError, CompilerErrorKind},
    program::Program,
    scope::variable::{Variable, VariableType},
};

impl Program {
    pub fn handle_assignment(&mut self, assignment: &Assignment) -> Result<Builder, CompilerError> {
        let expr = self.handle_expression(&assignment.value)?;

        let builder = Builder::new()
            .push(Procedure::new(
                assignment.identifier_pos.clone(),
                ProcedureKind::Comment(format!("Assignment: {}", assignment.identifier.clone())),
            ))
            .append(expr);

        if let Some(variable) = self.get_variable(&assignment.identifier) {
            if assignment.is_new {
                return Err(CompilerError::new(
                    assignment.identifier_pos.clone(),
                    CompilerErrorKind::RedeclaredVariable(
                        assignment.identifier.clone(),
                        variable.pos.clone(),
                    ),
                ));
            } else {
                let assignment_type = self.infer_type(&assignment.value)?;
                if assignment_type != variable.typ.to_keyword() {
                    return Err(CompilerError::new(
                        assignment.value_pos.clone(),
                        CompilerErrorKind::WrongAssignmentType {
                            got: assignment_type,
                            typ: variable.typ.to_keyword(),
                            declaration_pos: variable.pos.clone(),
                        },
                    ));
                }

                return Ok(builder.push(Procedure {
                    pos: assignment.identifier_pos.start..assignment.value_pos.end,
                    comment: Some(format!("Reassign: {}", assignment.identifier)),
                    kind: ProcedureKind::Assign(variable.location.clone()),
                }));
            }
        } else if !assignment.is_new {
            return Err(CompilerError::new(
                assignment.identifier_pos.clone(),
                CompilerErrorKind::UndefinedVariable(assignment.identifier.clone()),
            ));
        }

        if assignment.is_new {
            let infered = self.infer_type(&assignment.value)?;
            if let Some(typ) = assignment.typ {
                if typ != infered {
                    return Err(CompilerError::new(
                        assignment.value_pos.clone(),
                        CompilerErrorKind::WrongType {
                            got: infered,
                            expected: typ,
                        },
                    ));
                }
            }

            let variable_location = self.create_variable(
                assignment.identifier.clone(),
                Variable {
                    pos: assignment.identifier_pos.clone(),
                    typ: VariableType::Value(infered),
                },
            );

            return Ok(builder.push(Procedure {
                pos: assignment.identifier_pos.start..assignment.value_pos.end,
                comment: Some(format!("Assign: {}", assignment.identifier)),
                kind: ProcedureKind::Assign(variable_location.clone()),
            }));
        }

        todo!("this should not happen")
    }
}
