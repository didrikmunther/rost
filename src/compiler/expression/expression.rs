use crate::{
    compiler::{
        builder::Builder,
        definition::{Arithmetic, Procedure, ProcedureKind},
        error::CompilerError,
        program::Program,
    },
    lexer::Keyword,
    parser::definition::{Binary, Expression, ExpressionKind},
};

impl Program {
    pub fn handle_binary(
        &mut self,
        expression: &Expression,
        binary: &Binary,
    ) -> Result<Builder, CompilerError> {
        let operation = match binary.operator {
            Keyword::Plus => Arithmetic::Add,
            Keyword::Minus => Arithmetic::Subtract,
            Keyword::Asterix => Arithmetic::Multiply,
            Keyword::LessThan => Arithmetic::LessThan,
            Keyword::GreaterThan => Arithmetic::GreaterThan,
            Keyword::Equality => Arithmetic::Equality,
            _ => todo!(),
        };

        let builder = Builder::new()
            .append(self.handle_expression(&binary.right)?)
            .append(self.handle_expression(&binary.left)?)
            .push(Procedure::new(
                expression.pos.clone(),
                ProcedureKind::Arithmetic(operation),
            ));

        Ok(builder)
    }

    pub fn handle_expression(&mut self, expression: &Expression) -> Result<Builder, CompilerError> {
        match &expression.kind {
            ExpressionKind::FunctionCall(fcall) => self.handle_function_call(expression, fcall),
            ExpressionKind::Primary(primary) => self.handle_primary(expression, primary),
            ExpressionKind::Binary(binary) => self.handle_binary(expression, binary),
        }
    }
}
