use crate::lexer::{Block, Keyword};

use super::{
    definition::{Assignment, Expression, Statement, StatementKind},
    error::{ParserError, ParserErrorKind},
    parser::Parser,
    util::{get_block_identifier, get_expr_identifier},
};

static ALLOWED_TYPES: &[Keyword] = &[Keyword::Int, Keyword::Bool, Keyword::String];

impl<'a> Parser<'a> {
    fn parse_assignment_value(&mut self) -> Result<Option<Expression>, ParserError> {
        if let Some(_) = self.get(&[Keyword::Equals]) {
            Ok(Some(self.expression()?))
        } else {
            Ok(None)
        }
    }

    fn new_assignment(&mut self) -> Result<Statement, ParserError> {
        if let Some(left) = self.get(&[Keyword::Identifier]) {
            let mut assignment_type: Option<&Block> = None;

            if let Some(_) = self.get(&[Keyword::Colon]) {
                if let Some(typ) = self.get(ALLOWED_TYPES) {
                    assignment_type = Some(typ);
                } else {
                    return Err(ParserError::new(
                        self.peek_or_eof()?.pos.clone(),
                        ParserErrorKind::Expected(ALLOWED_TYPES),
                    ));
                }
            }

            if let Some(right) = self.parse_assignment_value()? {
                let identifier = match get_block_identifier(&left) {
                    Some(identifier) => identifier,
                    _ => {
                        return Err(ParserError::new(
                            left.pos.clone(),
                            ParserErrorKind::Expected(&[Keyword::Identifier]),
                        ))
                    }
                };

                return Ok(Statement {
                    pos: left.pos.start..right.pos.end,
                    kind: StatementKind::Assignment(Assignment {
                        is_new: true,
                        typ: assignment_type.map(|v| v.kind),
                        identifier,
                        identifier_pos: left.pos.clone(),
                        value_pos: right.pos.clone(),
                        value: Box::new(right),
                    }),
                });
            } else {
                Err(ParserError::new(
                    self.peek_or_eof()?.pos.clone(),
                    ParserErrorKind::Expected(&[Keyword::Equals]),
                ))
            }
        } else {
            return Err(ParserError::new(
                self.peek_or_eof()?.pos.clone(),
                ParserErrorKind::Expected(&[Keyword::Identifier]),
            ));
        }
    }

    pub fn assignment(&mut self) -> Result<Statement, ParserError> {
        if let Some(_) = self.get(&[Keyword::Let]) {
            return self.new_assignment();
        }

        let left = self.expression()?;
        if let Some(right) = self.parse_assignment_value()? {
            let identifier = match get_expr_identifier(&left) {
                Some(identifier) => identifier,
                _ => todo!(),
            };

            return Ok(Statement {
                pos: left.pos.start..right.pos.end,
                kind: StatementKind::Assignment(Assignment {
                    is_new: false,
                    typ: None,
                    identifier,
                    identifier_pos: left.pos.clone(),
                    value_pos: right.pos.clone(),
                    value: Box::new(right),
                }),
            });
        }

        // No assignment, simply an expression
        Ok(Statement {
            pos: left.pos.clone(),
            kind: StatementKind::Expression(left),
        })
    }
}
