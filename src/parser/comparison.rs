use crate::lexer::Keyword;

use super::{
    definition::{Binary, Expression, ExpressionKind},
    error::ParserError,
    parser::Parser,
};

impl<'a> Parser<'a> {
    pub fn comparison(&mut self) -> Result<Expression, ParserError> {
        let left = self.addition()?;

        while let Some(block) =
            self.get(&[Keyword::LessThan, Keyword::GreaterThan, Keyword::Equality])
        {
            let right = self.addition()?;
            let pos = left.pos.start..right.pos.end;

            return Ok(Expression {
                pos: pos.clone(),
                kind: ExpressionKind::Binary(Binary {
                    left: Box::new(left),
                    right: Box::new(right),
                    operator: block.kind,
                }),
            });
        }

        Ok(left)
    }
}
