use crate::lexer::Keyword;

use super::{
    definition::{Expression, ExpressionKind, FunctionCall},
    error::{ParserError, ParserErrorKind},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn function_call_from(&mut self, expr: Expression) -> Result<Expression, ParserError> {
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
                                left: Box::new(expr),
                                args,
                                args_pos: open.pos.start..close.pos.end,
                            }),
                        });
                    }
                }

                args.push(self.expression()?);
            }
        }

        Ok(expr)
    }

    pub fn function_call(&mut self) -> Result<Expression, ParserError> {
        let expr = self.index()?;

        if expr.get_string().is_some() {
            return self.function_call_from(expr);
        }

        Ok(expr)
    }
}
