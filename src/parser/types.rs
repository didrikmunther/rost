use std::{
    fmt::{self, Debug, Display},
    ops::Range,
};

use crate::{
    lexer::{Keyword, Token},
    parser_todo,
};

use super::{error::ParserError, Parser};

#[derive(Debug, Clone)]
pub enum TypeKind {
    Identifier(String),
    Composed {
        identifier: String,
        children: Vec<Type>,
    },
    Pointer(Box<Type>),
}

#[derive(Debug, Clone)]
pub struct Type {
    pub kind: TypeKind,
    pub pos: Range<usize>,
}

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            TypeKind::Identifier(identifier) => write!(f, "{}", identifier),
            TypeKind::Composed {
                identifier,
                children,
            } => {
                write!(f, "{}<", identifier)?;
                for (i, child) in children.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", child)?;
                }
                write!(f, ">")
            }
            TypeKind::Pointer(child) => write!(f, "&{}", child),
        }
    }
}

impl<'a> Parser<'a> {
    pub fn parse_type(&mut self) -> Result<Type, ParserError> {
        let next = self.expect(&[Keyword::Identifier, Keyword::Ampersand])?;

        let identifier = match &next.token {
            Token::Keyword(Keyword::Ampersand) => {
                let child = self.parse_type()?;

                return Ok(Type {
                    pos: next.pos.start..child.pos.end,
                    kind: TypeKind::Pointer(Box::new(child)),
                });
            }
            Token::Identifier(identifier) => identifier.clone(),
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
                pos: next.pos.start..children.last().unwrap().pos.end,
                kind: TypeKind::Composed {
                    identifier,
                    children,
                },
            })
        } else {
            Ok(Type {
                kind: TypeKind::Identifier(identifier),
                pos: next.pos.clone(),
            })
        }
    }
}
