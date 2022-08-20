use crate::parser::definition::Assignment;

use super::{
    builder::Builder,
    definition::{Procedure, ProcedureKind},
    error::{CompilerError, CompilerErrorKind},
    program::{Program, Variable},
};

impl Program {
    pub fn handle_assignment(&mut self, assignment: &Assignment) -> Result<Builder, CompilerError> {
        let expr = self.handle_expression(&assignment.value)?;
        let mut builder = Builder::new().append(expr);

        if let Some(variable) = self.variables.get(&assignment.identifier) {
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

                if assignment_type != variable.typ {
                    return Err(CompilerError::new(
                        assignment.value_pos.clone(),
                        CompilerErrorKind::WrongAssignmentType {
                            got: assignment_type,
                            typ: variable.typ,
                            declaration_pos: variable.pos.clone(),
                        },
                    ));
                }

                return Ok(builder.push(Procedure {
                    pos: assignment.identifier_pos.start..assignment.value_pos.end,
                    kind: ProcedureKind::Reassign(variable.stack_pos),
                    comment: Some(format!(
                        "Reassign: {}, stack: {}",
                        assignment.identifier, variable.stack_pos
                    )),
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

            self.variables.insert(
                assignment.identifier.clone(),
                Variable {
                    pos: assignment.identifier_pos.clone(),
                    typ: infered,
                    stack_pos: self.stack_pos,
                },
            );
        }

        builder = builder.push(Procedure::new(
            assignment.identifier_pos.clone(),
            ProcedureKind::Comment(format!(
                "Assignment: {}, stack: {}",
                assignment.identifier.clone(),
                self.stack_pos
            )),
        ));

        self.stack_pos += 1;

        Ok(builder)
    }
}
