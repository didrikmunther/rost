use crate::parser::definition::{Assignment as ParsedAssignment, Statement};

use super::{
    definition::{Assignment, Procedure, ProcedureKind},
    error::CompilerError,
    program::Program,
};

impl Program {
    pub fn handle_assignment(
        &mut self,
        statement: &Statement,
        assignment: &ParsedAssignment,
    ) -> Result<Procedure, CompilerError> {
        self.variables.insert(assignment.identifier.clone(), self.stack_pos);
        self.stack_pos += 1;

        Ok(Procedure {
            pos: statement.pos.clone(),
            kind: ProcedureKind::Assignment(Assignment {
                identifier: assignment.identifier.clone(),
            }),
        })
    }
}
