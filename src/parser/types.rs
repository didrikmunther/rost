use std::{
    fmt::{Debug, Display},
    ops::Range,
};

use crate::{
    lexer::{lex, Keyword, Token},
    parser_todo,
};

use super::{error::ParserError, Parser};

#[derive(Debug, Clone)]
pub enum TypeIdentifier {
    Primitive(Keyword),
    Struct(String),
}

#[derive(Debug, Clone)]
pub struct Type {
    pub identifier: TypeIdentifier,
    pub pos: Range<usize>,
    pub children: Option<Vec<Type>>,
}

impl<'a> Parser<'a> {
    pub fn parse_type(&mut self) -> Result<Type, ParserError> {
        let next = self.peek_or_eof()?;
        self.advance();

        let identifier = match &next.token {
            Token::Keyword(keyword) => match keyword {
                Keyword::Int | Keyword::Bool | Keyword::Char | Keyword::Pointer => {
                    TypeIdentifier::Primitive(*keyword)
                }
                Keyword::Ampersand => {
                    let child = self.parse_type()?;

                    return Ok(Type {
                        identifier: TypeIdentifier::Primitive(Keyword::Pointer),
                        pos: next.pos.start..child.pos.end,
                        children: Some(vec![child]),
                    });
                }
                _ => return parser_todo!(next.pos.clone(), "Unknown type"),
            },
            Token::Identifier(identifier) => TypeIdentifier::Struct(identifier.clone()),
            _ => return parser_todo!(next.pos.clone(), "Unknown type"),
        };

        if let Some(lt) = self.get(&[Keyword::LessThan]) {
            let mut children = vec![self.parse_type()?];
            while self.get(&[Keyword::Comma]).is_some() {
                children.push(self.parse_type()?);
            }

            if self.get(&[Keyword::GreaterThan]).is_none() {
                return parser_todo!(lt.pos.clone(), "Unclosed type");
            }

            Ok(Type {
                identifier,
                pos: next.pos.start..children.last().unwrap().pos.end,
                children: Some(children),
            })
        } else {
            Ok(Type {
                identifier,
                pos: next.pos.clone(),
                children: None,
            })
        }
    }
}
