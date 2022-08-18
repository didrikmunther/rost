use crate::lexer::{Keyword, Token};

use super::{
    definition::{Expression, ExpressionKind, Primary},
    error::{ParserError, ParserErrorKind},
    parser::Parser,
};

impl<'a> Parser<'a> {
    pub fn primary(&mut self) -> Result<Expression, ParserError> {
        if let Some(block) = self.get(&[Keyword::Literal, Keyword::Identifier]) {
            return Ok(Expression {
                pos: block.pos.clone(),
                kind: ExpressionKind::Primary(match &block.token {
                    Token::Identifier(identifier) => Primary::Identifier(identifier.clone()),
                    Token::Literal(literal) => Primary::Literal(literal.clone()),
                    _ => {
                        return Err(ParserError::new(
                            block.pos.clone(),
                            ParserErrorKind::Unknown,
                        ))
                    }
                }),
            });
        }

        self.unexpected()
    }
}
