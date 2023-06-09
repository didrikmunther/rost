use std::collections::HashMap;

use crate::lexer::{Keyword, Token};

use super::{
    definition::{Expression, ExpressionKind, StructConstruction, StructConstructionField},
    error::{ParserError, ParserErrorKind},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn struct_contruction(&mut self) -> Result<Expression, ParserError> {
        let (struct_identifier, open, identifier) = match (self.peek(), self.peek_offset(1)) {
            (Some(struct_identifier), Some(open)) => match (&struct_identifier.token, open.kind) {
                (Token::Identifier(identifier), Keyword::BraceLeft) => {
                    (struct_identifier, open, identifier)
                }
                _ => return self.reference(),
            },
            _ => return self.reference(),
        };

        self.advance_n(2);

        let mut fields = HashMap::new();

        loop {
            if self.is_end() {
                return Err(ParserError::new(
                    open.pos.clone(),
                    ParserErrorKind::UnterminatedPair(Keyword::BraceLeft),
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
                    println!("here: {struct_identifier:#?}");
                    return Err(ParserError::new(
                        self.peek().unwrap().pos.clone(),
                        ParserErrorKind::Expected(&[Keyword::Identifier]),
                    ));
                };

            self.expect(&[Keyword::Colon])?;

            if let Some(existing_field) = fields.insert(
                field_identifier.clone(),
                StructConstructionField {
                    expr: self.expression()?,
                    pos: field_identifier_pos.clone(),
                },
            ) {
                return Err(ParserError::new(
                    field_identifier_pos.clone(),
                    ParserErrorKind::FieldAlreadyDefined {
                        identifier: field_identifier.clone(),
                        identifier_pos: existing_field.pos,
                    },
                ));
            }

            if self.get(&[Keyword::Comma]).is_none()
                || self.get_peek(&[Keyword::BraceRight]).is_some()
            {
                break;
            }
        }

        let close = self.expect(&[Keyword::BraceRight])?;

        Ok(Expression {
            pos: struct_identifier.pos.start..close.pos.end,
            kind: ExpressionKind::StructConstruction(StructConstruction {
                identifier: identifier.clone(),
                identifier_pos: struct_identifier.pos.clone(),
                fields,
            }),
        })
    }
}
