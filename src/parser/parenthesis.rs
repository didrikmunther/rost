use crate::lexer::Keyword;

use super::{
    definition::Expression,
    error::{ParserError, ParserErrorKind},
    parser::Parser,
};

impl<'a> Parser<'a> {
    pub fn parenthesis(&mut self) -> Result<Expression, ParserError> {
        if let Some(parenthesis) = self.get(&[Keyword::ParLeft]) {
            let expr = self.expression()?;
            if let None = self.get(&[Keyword::ParRight]) {
                return Err(ParserError::new(
                    parenthesis.pos.clone(),
                    ParserErrorKind::UnterminatedParenthesis,
                ));
            }

            return Ok(expr);
        }

        self.unexpected()
    }
}
