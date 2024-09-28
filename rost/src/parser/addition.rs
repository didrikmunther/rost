use crate::lexer::Keyword;

use super::{
    definition::{Binary, Expression, ExpressionKind},
    error::ParserError,
    Parser,
};

impl<'a> Parser<'a> {
    pub fn addition(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.multiplication()?;

        while let Some(operator) = self.get(&[Keyword::Plus, Keyword::Minus]) {
            let right = self.multiplication()?;
            let pos = expr.pos.start..right.pos.end;

            expr = Expression {
                pos: pos.clone(),
                kind: ExpressionKind::Binary(Binary {
                    left: Box::new(expr),
                    right: Box::new(right),
                    operator: operator.kind,
                    operator_pos: operator.pos.clone(),
                }),
            }
        }

        Ok(expr)
    }
}
