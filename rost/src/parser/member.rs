use crate::lexer::Keyword;

use super::{
    definition::{Expression, ExpressionKind, MemberAccess},
    error::ParserError,
    util::get_expr_identifier,
    Parser,
};

impl<'a> Parser<'a> {
    pub fn member(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.function_call()?;

        while self.get(&[Keyword::Dot]).is_some() {
            let next = self.primary()?;

            if let Some(identifier) = get_expr_identifier(&next) {
                expr = Expression {
                    pos: expr.pos.start..next.pos.end,
                    kind: ExpressionKind::MemberAccess(MemberAccess {
                        left: Box::new(expr),
                        member: identifier,
                    }),
                };
            } else {
                todo!("Member must be a string literal");
            }

            expr = self.index_from(expr)?;
            expr = self.function_call_from(expr)?;
        }

        Ok(expr)
    }
}
