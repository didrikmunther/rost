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

        let identifier = match &next.token {
            Token::Keyword(keyword) => match keyword {
                Keyword::Int | Keyword::Bool | Keyword::Char | Keyword::Pointer => {
                    TypeIdentifier::Primitive(keyword.clone())
                }
                _ => todo!("Unknown type"),
            },
            Token::Identifier(_identifier) => todo!(),
            _ => todo!("Unknown type"),
        };

        if let Some(_) = self.get(&[Keyword::LessThan]) {
            let mut children = vec![Box::new(self.parse_type()?)];
            while let Some(_) = self.get(&[Keyword::Comma]) {
                children.push(Box::new(self.parse_type()?));
            }

            if let None = self.get(&[Keyword::GreaterThan]) {
                todo!("Unclosed type");
            }

            Ok(Type {
                identifier,
                children: Some(children),
            })
        } else {
            Ok(Type {
                identifier,
                children: None,
            })
        }
    }
}
