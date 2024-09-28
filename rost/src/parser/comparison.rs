use crate::lexer::Keyword;

use super::{
    definition::{Binary, Expression, ExpressionKind},
    error::ParserError,
    Parser,
};

impl<'a> Parser<'a> {
    pub fn comparison(&mut self) -> Result<Expression, ParserError> {
        let left = self.addition()?;

        if let Some(operator) =
            self.get(&[Keyword::LessThan, Keyword::GreaterThan, Keyword::Equality])
        {
            let right = self.addition()?;
            let pos = left.pos.start..right.pos.end;

            return Ok(Expression {
                pos,
                kind: ExpressionKind::Binary(Binary {
                    left: Box::new(left),
                    right: Box::new(right),
                    operator: operator.kind,
                    operator_pos: operator.pos.clone(),
                }),
            });
        }

        Ok(left)
    }
}
