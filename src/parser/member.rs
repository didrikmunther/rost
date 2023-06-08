use crate::lexer::Keyword;

use super::{
    definition::{Expression, ExpressionKind, MemberAccess},
    error::ParserError,
    util::get_expr_identifier,
    Parser,
};

impl<'a> Parser<'a> {
    pub fn member(&mut self) -> Result<Expression, ParserError> {
        let expr = self.index()?;

        if self.get(&[Keyword::Dot]).is_some() {
            let next = self.index()?;

            if let Some(identifier) = get_expr_identifier(&next) {
                return Ok(Expression {
                    pos: expr.pos.start..next.pos.end,
                    kind: ExpressionKind::MemberAccess(MemberAccess {
                        left: Box::new(expr),
                        member: identifier,
                    }),
                });
            }
        }

        Ok(expr)
    }
}
