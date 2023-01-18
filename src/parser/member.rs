use crate::lexer::Keyword;

use super::{
    definition::{Expression, ExpressionKind, MemberAccess},
    error::ParserError,
    parser::Parser,
};

impl<'a> Parser<'a> {
    pub fn member(&mut self) -> Result<Expression, ParserError> {
        let expr = self.function_call()?;

        if let Some(_) = self.get(&[Keyword::Dot]) {
            let next = self.member()?;

            return Ok(Expression {
                pos: expr.pos.start..next.pos.end,
                kind: ExpressionKind::MemberAccess(MemberAccess {
                    left: Box::new(expr),
                    right: Box::new(next),
                }),
            });
        }

        Ok(expr)
    }
}
