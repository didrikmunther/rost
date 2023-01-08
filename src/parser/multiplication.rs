use crate::lexer::Keyword;

use super::{
    definition::{Binary, Expression, ExpressionKind, Unary},
    error::ParserError,
    parser::Parser,
};

impl<'a> Parser<'a> {
    pub fn deref(&mut self) -> Result<Expression, ParserError> {
        if let Some(ampersand) = self.get(&[Keyword::Ampersand]) {
            let expr = self.primary()?;

            return Ok(Expression {
                pos: ampersand.pos.clone(),
                kind: ExpressionKind::Unary(Unary {
                    expr: Box::new(expr),
                    operator: Keyword::Ampersand,
                }),
            });
        }

        self.function_call()
    }

    pub fn multiplication(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.deref()?;

        while let Some(block) = self.get(&[Keyword::Asterix, Keyword::Slash]) {
            let right = self.primary()?;
            let pos = expr.pos.start..right.pos.end;

            expr = Expression {
                // typ: expr.typ,
                pos: pos.clone(),
                kind: ExpressionKind::Binary(Binary {
                    left: Box::new(expr),
                    right: Box::new(right),
                    operator: block.kind,
                }),
            }
        }

        Ok(expr)
    }
}
