use crate::lexer::{Keyword, Literal, Token};

use super::{
    definition::{Expression, ExpressionKind, Primary},
    error::{ParserError, ParserErrorKind},
    parser::Parser,
};

impl<'a> Parser<'a> {
    pub fn primary(&mut self) -> Result<Expression, ParserError> {
        if let Some(block) = self.get(&[Keyword::Literal, Keyword::Identifier]) {
            let (_, kind) = match &block.token {
                Token::Identifier(identifier) => {
                    (Keyword::Identifier, Primary::Identifier(identifier.clone()))
                }
                Token::Literal(literal) => {
                    let typ = match literal {
                        Literal::String(_) => Keyword::String,
                        Literal::Int(_) => Keyword::Int,
                        Literal::Bool(_) => Keyword::Bool,
                    };

                    (typ, Primary::Literal(literal.clone()))
                }
                _ => {
                    return Err(ParserError::new(
                        block.pos.clone(),
                        ParserErrorKind::Unknown,
                    ));
                }
            };

            return Ok(Expression {
                pos: block.pos.clone(),
                kind: ExpressionKind::Primary(kind),
            });
        }

        self.parenthesis()
    }
}
