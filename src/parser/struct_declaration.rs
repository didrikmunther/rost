use std::collections::HashMap;

use crate::lexer::{Keyword, Token};

use super::{
    definition::{Declaration, DeclarationKind, StructDeclaration, StructField},
    error::{ParserError, ParserErrorKind},
    parser::Parser,
    util::get_block_identifier,
};

impl<'a> Parser<'a> {
    pub fn struct_declaration(&mut self) -> Result<Declaration, ParserError> {
        if let Some(_) = self.get(&[Keyword::Struct]) {
            let struct_identifier = self.expect(&[Keyword::Identifier])?;
            let identifier = match get_block_identifier(&struct_identifier) {
                Some(identifier) => identifier,
                _ => {
                    return Err(ParserError::new(
                        struct_identifier.pos.clone(),
                        ParserErrorKind::Expected(&[Keyword::Identifier]),
                    ))
                }
            };

            let open = self.expect(&[Keyword::BracketLeft])?;
            let mut fields = HashMap::new();

            loop {
                if self.is_end() {
                    return Err(ParserError::new(
                        open.pos.clone(),
                        ParserErrorKind::UnterminatedPair(Keyword::BracketLeft),
                    ));
                }

                let (field_identifier, field_identifier_pos) =
                    if let Some(identifier) = self.get(&[Keyword::Identifier]) {
                        if let Token::Identifier(ref s) = identifier.token {
                            (s, &identifier.pos)
                        } else {
                            return Err(ParserError::new(
                                struct_identifier.pos.clone(),
                                ParserErrorKind::Expected(&[Keyword::Identifier]),
                            ));
                        }
                    } else {
                        return Err(ParserError::new(
                            struct_identifier.pos.clone(),
                            ParserErrorKind::Expected(&[Keyword::Identifier]),
                        ));
                    };

                self.expect(&[Keyword::Colon])?;
                let typ = self.parse_type()?;

                fields.insert(
                    field_identifier.clone(),
                    StructField {
                        typ,
                        pos: field_identifier_pos.clone(),
                    },
                );

                if let None = self.get(&[Keyword::Comma]) {
                    break;
                }

                // Allow trailing colon.
                if let Some(_) = self.get_peek(&[Keyword::BracketRight]) {
                    break;
                }
            }

            let close = self.expect(&[Keyword::BracketRight])?;

            return Ok(Declaration {
                pos: struct_identifier.pos.start..close.pos.end,
                kind: DeclarationKind::StructDeclaration(StructDeclaration { identifier, fields }),
            });
        }

        self.function_declaration()
    }
}
