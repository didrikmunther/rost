use std::{
    fmt::{Debug, Display},
    ops::Range,
};

use crate::{
    lexer::{lex, Keyword, Token},
    parser_todo,
};

use super::{error::ParserError, Parser};

#[derive(Debug, Clone, PartialEq)]
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

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.children == other.children
    }
}

impl Type {
    pub fn is_type(&self, typ: &'static str) -> bool {
        let lexed = &lex(typ).unwrap_or_else(|_| panic!("Type invalid: {typ}"));
        Parser::new(lexed)
            .parse_type()
            .unwrap_or_else(|_| panic!("Type invalid: {typ}"))
            .eq(self)
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let identifier = match self.identifier {
            TypeIdentifier::Primitive(keyword) => match keyword {
                Keyword::Char => "char".to_string(),
                Keyword::Int => "int".to_string(),
                keyword => format!("{keyword:?}"),
            },
            TypeIdentifier::Struct(ref identifier) => identifier.clone(),
        };

        if let TypeIdentifier::Primitive(Keyword::Pointer) = self.identifier {
            let child = self.children.as_ref().unwrap().first().unwrap();
            f.write_str("&")?;
            std::fmt::Display::fmt(&child, f)?;
        } else {
            f.write_fmt(format_args!("{identifier}"))?;

            if let Some(ref children) = self.children {
                f.write_str("<")?;
                for child in children {
                    std::fmt::Display::fmt(&child, f)?;
                }
                f.write_str(">")?;
            }
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
