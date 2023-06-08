use crate::lexer::Keyword;

use super::{
    definition::{Expression, ExpressionKind, FunctionCall},
    error::{ParserError, ParserErrorKind},
    util::get_expr_identifier,
    Parser,
};

impl<'a> Parser<'a> {
    pub fn function_call(&mut self) -> Result<Expression, ParserError> {
        let expr = self.member()?;

        if let Some(identifier) = get_expr_identifier(&expr) {
            if let Some(open) = self.get(&[Keyword::ParLeft]) {
                let mut args = Vec::new();

                loop {
                    if self.is_end() {
                        return Err(ParserError::new(
                            open.pos.clone(),
                            ParserErrorKind::UnterminatedPair(Keyword::ParLeft),
                        ));
                    }

                    if self.get(&[Keyword::Comma]).is_none() {
                        if let Some(close) = self.get(&[Keyword::ParRight]) {
                            return Ok(Expression {
                                pos: expr.pos.start..close.pos.end,
                                kind: ExpressionKind::FunctionCall(FunctionCall {
                                    identifier,
                                    args,
                                    identifier_pos: expr.pos.clone(),
                                    parameters_pos: open.pos.start..close.pos.end,
                                }),
                            });
                        }
                    }

                    args.push(self.expression()?);
                }
            }
        }

        Ok(expr)
    }
}
