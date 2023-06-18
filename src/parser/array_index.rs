use crate::lexer::Keyword;

use super::{
    definition::{ArrayIndex, Expression, ExpressionKind},
    error::{ParserError, ParserErrorKind},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn index_from(&mut self, mut expr: Expression) -> Result<Expression, ParserError> {
        while let Some(open) = self.get(&[Keyword::BracketLeft]) {
            let index = self.expression()?;

            if let Some(close) = self.get(&[Keyword::BracketRight]) {
                expr = Expression {
                    pos: expr.pos.start..close.pos.end,
                    kind: ExpressionKind::ArrayIndex(ArrayIndex {
                        left: Box::new(expr),
                        index: Box::new(index),
                    }),
                };
            } else {
                return Err(ParserError::new(
                    open.pos.clone(),
                    ParserErrorKind::UnterminatedPair(Keyword::BracketLeft),
                ));
            }
        }

        Ok(expr)
    }

    pub fn index(&mut self) -> Result<Expression, ParserError> {
        let expr = self.primary()?;

        self.index_from(expr)
    }
}
