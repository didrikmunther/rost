use crate::parser::definition::Assignment;

use super::{definition::Procedure, error::CompilerError, program::Program};

impl Program {
    pub fn handle_assignment(
        &mut self,
        _assignment: &Assignment,
    ) -> Result<Procedure, CompilerError> {
        todo!()
    }
}
