use crate::lexer::Keyword;

use super::{
    definition::Expression,
    error::{ParserError, ParserErrorKind},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn parenthesis(&mut self) -> Result<Expression, ParserError> {
        if let Some(parenthesis) = self.get(&[Keyword::ParLeft]) {
            let expr = self.expression()?;
            if self.get(&[Keyword::ParRight]).is_none() && self.is_end() {
                return Err(ParserError::new(
                    parenthesis.pos.clone(),
                    ParserErrorKind::UnterminatedPair(Keyword::ParLeft),
                ));
            }

            return Ok(expr);
        }

        self.unexpected()
    }
}
