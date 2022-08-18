use crate::lexer::{Keyword, Literal};

use super::{
    definition::{Assignment, Expression, ExpressionKind, Primary, Statement, StatementKind},
    error::{ParserError, ParserErrorKind},
    parser::Parser,
    util::{get_block_identifier, get_expr_identifier},
};

fn infer_type(expr: &Expression) -> Option<Keyword> {
    match &expr.kind {
        ExpressionKind::Primary(primary) => match primary {
            Primary::Literal(literal) => match literal {
                Literal::Int(_) => Some(Keyword::Int),
                Literal::Bool(_) => Some(Keyword::Bool),
                Literal::String(_) => Some(Keyword::String),
            },
            _ => None,
        },
        _ => None,
    }
}

static ALLOWED_TYPES: &[Keyword] = &[Keyword::Int];

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
            let mut assignment_type: Option<Keyword> = None;

            if let Some(_) = self.get(&[Keyword::Colon]) {
                if let Some(typ) = self.get(ALLOWED_TYPES) {
                    assignment_type = Some(typ.kind);
                } else {
                    return Err(ParserError::new(
                        self.peek_or_eof()?.pos.clone(),
                        ParserErrorKind::Expected(ALLOWED_TYPES),
                    ));
                }
            }

            if let Some(right) = self.parse_assignment_value()? {
                if let None = assignment_type {
                    assignment_type = infer_type(&right);
                }

                if let Some(typ) = assignment_type {
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
                            typ: Some(typ),
                            identifier,
                            identifier_pos: left.pos.clone(),
                            value_pos: right.pos.clone(),
                            value: Box::new(right),
                        }),
                    });
                } else {
                    return Err(ParserError::new(
                        left.pos.clone(),
                        ParserErrorKind::CannotInferType,
                    ));
                }
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
