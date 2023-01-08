use crate::{
    compiler::{builder::Builder, error::CompilerError, program::Program},
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
            Primary::Identifier(identifier) => self.handle_identifier(expression, identifier, false),
        }
    }
}
