use crate::lexer::Keyword;

use super::{
    definition::{ArrayIndex, Expression, ExpressionKind},
    error::{ParserError, ParserErrorKind},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn index(&mut self) -> Result<Expression, ParserError> {
        let expr = self.primary()?;

        if let Some(open) = self.get(&[Keyword::BracketLeft]) {
            let index = self.expression()?;

            if let Some(close) = self.get(&[Keyword::BracketRight]) {
                return Ok(Expression {
                    pos: expr.pos.start..close.pos.end,
                    kind: ExpressionKind::ArrayIndex(ArrayIndex {
                        left: Box::new(expr),
                        index: Box::new(index),
                    }),
                });
            } else {
                return Err(ParserError::new(
                    open.pos.clone(),
                    ParserErrorKind::UnterminatedPair(Keyword::BracketLeft),
                ));
            }
        }

        Ok(expr)
    }
}
