use crate::parser::definition::Assignment;

use super::{
    builder::Builder,
    definition::{Procedure, ProcedureKind},
    error::{CompilerError, CompilerErrorKind},
    program::{Program, Variable},
};

impl Program {
    pub fn handle_assignment(&mut self, assignment: &Assignment) -> Result<Builder, CompilerError> {
        let mut builder = Builder::new().append(self.handle_expression(&assignment.value)?);

        if let Some(variable) = self.variables.get(&assignment.identifier) {
            if assignment.is_new {
                return Err(CompilerError::new(
                    assignment.identifier_pos.clone(),
                    CompilerErrorKind::RedeclaredVariable(
                        assignment.identifier.clone(),
                        variable.pos.clone(),
                    ),
                ));
            }
        } else if !assignment.is_new {
            return Err(CompilerError::new(
                assignment.identifier_pos.clone(),
                CompilerErrorKind::UndefinedVariable(assignment.identifier.clone()),
            ));
        }

        self.variables.insert(
            assignment.identifier.clone(),
            Variable {
                pos: assignment.identifier_pos.clone(),
                stack_pos: self.stack_pos,
            },
        );

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
