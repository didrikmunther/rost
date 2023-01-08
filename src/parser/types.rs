use crate::lexer::{Keyword, Token};

use super::{error::ParserError, parser::Parser};

#[derive(Debug, Clone)]
pub enum TypeIdentifier {
    Primitive(Keyword),
}

#[derive(Debug, Clone)]
pub struct Type {
    pub identifier: TypeIdentifier,
    pub children: Option<Vec<Box<Type>>>,
}

impl<'a> Parser<'a> {
    pub fn parse_type(&mut self) -> Result<Type, ParserError> {
        let next = self.peek_or_eof()?;
        self.advance();

        Ok(match &next.token {
            Token::Keyword(keyword) => match keyword {
                Keyword::Int | Keyword::Bool => Type {
                    identifier: TypeIdentifier::Primitive(keyword.clone()),
                    children: None,
                },
                _ => todo!("Unknown type"),
            },
            Token::Identifier(identifier) => todo!(),
            _ => todo!("Unknown type"),
        })
    }
}
