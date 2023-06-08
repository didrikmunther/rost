use crate::lexer::Keyword;

use super::{
    definition::{Binary, Expression, ExpressionKind},
    error::ParserError,
    Parser,
};

impl<'a> Parser<'a> {
    pub fn multiplication(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.struct_contruction()?;

        while let Some(block) = self.get(&[Keyword::Asterix, Keyword::Slash]) {
            let right = self.primary()?;
            let pos = expr.pos.start..right.pos.end;

            expr = Expression {
                pos: pos.clone(),
                kind: ExpressionKind::Binary(Binary {
                    left: Box::new(expr),
                    right: Box::new(right),
                    operator: block.kind,
                    operator_pos: block.pos.clone(),
                }),
            }
        }

        Ok(expr)
    }
}
