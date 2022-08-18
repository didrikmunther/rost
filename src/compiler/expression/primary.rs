use crate::{
    compiler::{
        builder::Builder,
        error::{CompilerError, CompilerErrorKind},
        program::Program,
    },
    parser::definition::{Expression, Primary},
};

impl Program {
    pub fn handle_primary(
        &mut self,
        expression: &Expression,
        primary: &Primary,
    ) -> Result<Builder, CompilerError> {
        match primary {
            Primary::Literal(literal) => self.handle_literal(expression, literal),
            _ => {
                return Err(CompilerError::new(
                    expression.pos.clone(),
                    CompilerErrorKind::Unimplemented(format!("{:?}", primary)),
                ))
            }
        }
    }
}
