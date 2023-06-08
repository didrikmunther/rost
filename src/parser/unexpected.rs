use super::{
    definition::Expression,
    error::{ParserError, ParserErrorKind},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn unexpected(&mut self) -> Result<Expression, ParserError> {
        if let Some(block) = self.peek() {
            Err(ParserError::new(
                block.pos.clone(),
                ParserErrorKind::UnexpectedToken(block.token.clone()),
            ))
        } else {
            Err(ParserError::new(
                self.get_at(self.index - 1)
                    .map(|v| v.pos.clone())
                    .unwrap_or(0..0),
                ParserErrorKind::UnexpectedEOF,
            ))
        }
    }
}
