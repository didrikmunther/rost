use std::{
    fmt::{Debug, Display},
    ops::Range,
};

use crate::{
    lexer::{Keyword, Token},
    parser_todo,
};

use super::{error::ParserError, Parser};

#[derive(Debug, Clone, PartialEq)]
pub enum TypeIdentifier {
    Primitive(Keyword),
    Struct(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub identifier: TypeIdentifier,
    pub identifier_pos: Range<usize>,
    pub pos: Range<usize>,
    pub children: Option<Vec<Type>>,
}

impl Type {
    pub fn get_identifier(&self) -> Option<&str> {
        if let TypeIdentifier::Struct(identifier) = &self.identifier {
            Some(identifier)
        } else {
            None
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TypeIdentifier::*;

        let identifier = match &self.identifier {
            Primitive(keyword) => return Display::fmt(&keyword, f),
            Struct(identifier) => identifier,
        };

        write!(f, "{identifier}")?;

        if let Some(children) = &self.children {
            write!(f, "<")?;

            let mut children = children.iter().peekable();
            while let Some(child) = children.next() {
                Display::fmt(&child, f)?;
                if children.peek().is_some() {
                    write!(f, ", ")?;
                }
            }

            write!(f, ">")?;
        }

        Ok(())
    }
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
                        identifier_pos: next.pos.clone(),
                        pos: next.pos.start..child.pos.end,
                        children: Some(vec![child]),
                    });
                }
                _ => return parser_todo!(next.pos.clone(), format!("Unknown type {keyword:?}")),
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
                identifier_pos: next.pos.clone(),
                pos: next.pos.start..children.last().unwrap().pos.end,
                children: Some(children),
            })
        } else {
            Ok(Type {
                identifier,
                identifier_pos: next.pos.clone(),
                pos: next.pos.clone(),
                children: None,
            })
        }
    }
}
