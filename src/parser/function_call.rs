use crate::lexer::Keyword;

use super::{
    definition::{Expression, ExpressionKind, FunctionCall},
    error::{ParserError, ParserErrorKind},
    parser::Parser, util::get_expr_identifier,
};

impl<'a> Parser<'a> {
    pub fn function_call(&mut self) -> Result<Expression, ParserError> {
        let expr = self.primary()?;

        if let Some(identifier) = get_expr_identifier(&expr) {
            if let Some(open) = self.get(&[Keyword::ParLeft]) {
                let mut args = vec![];
    
                loop {
                    if self.is_end() {
                        return Err(ParserError::new(
                            open.pos.clone(),
                            ParserErrorKind::UnterminatedParenthesis,
                        ));
                    }
    
                    if let None = self.get(&[Keyword::Comma]) {
                        if let Some(close) = self.get(&[Keyword::ParRight]) {
                            return Ok(Expression {
                                pos: open.pos.start..close.pos.start,
                                kind: ExpressionKind::FunctionCall(FunctionCall {
                                    identifier,
                                    args,
                                }),
                            });
                        }
                    }
    
                    args.push(Box::new(self.expression()?));
                }
            }
        }

        Ok(expr)
    }
}
