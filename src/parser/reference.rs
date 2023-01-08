use crate::lexer::Keyword;

use super::{
    definition::{Expression, ExpressionKind, Unary},
    error::ParserError,
    parser::Parser,
};

impl<'a> Parser<'a> {
    /// Handle reference and dereferencing
    // todo: should this be before or after function_call?
    pub fn reference(&mut self) -> Result<Expression, ParserError> {
        if let Some(operation) = self.get(&[Keyword::Ampersand, Keyword::Asterix]) {
            let expr = self.reference()?;

            println!("pos: {:?}, expr.pos: {:?}", operation.pos.start, expr.pos);

            return Ok(Expression {
                pos: operation.pos.start..expr.pos.end,
                kind: ExpressionKind::Unary(Unary {
                    expr: Box::new(expr),
                    operator: operation.kind,
                }),
            });
        }

        self.function_call()
    }
}
