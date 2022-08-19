use crate::lexer::Keyword;

use super::{
    definition::{Binary, Expression, ExpressionKind},
    error::ParserError,
    parser::Parser,
};

impl<'a> Parser<'a> {
    pub fn multiplication(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.function_call()?;

        while let Some(block) = self.get(&[Keyword::Asterix]) {
            let right = self.primary()?;
            let pos = expr.pos.start..right.pos.end;

            expr = Expression {
                typ: expr.typ,
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
