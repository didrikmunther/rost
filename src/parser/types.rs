use crate::{
    lexer::{Keyword, Token},
    parser_todo,
};

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
                Keyword::Ampersand => {
                    return Ok(Type {
                        identifier: TypeIdentifier::Primitive(Keyword::Pointer),
                        children: Some(vec![Box::new(self.parse_type()?)]),
                    })
                }
                _ => return parser_todo!(next.pos.clone(), "Unknown type"),
            },
            Token::Identifier(_identifier) => todo!(),
            _ => return parser_todo!(next.pos.clone(), "Unknown type"),
        };

        if let Some(lt) = self.get(&[Keyword::LessThan]) {
            let mut children = vec![Box::new(self.parse_type()?)];
            while let Some(_) = self.get(&[Keyword::Comma]) {
                children.push(Box::new(self.parse_type()?));
            }

            if let None = self.get(&[Keyword::GreaterThan]) {
                return parser_todo!(lt.pos.clone(), "Unclosed type");
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
