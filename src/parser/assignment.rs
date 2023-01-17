use crate::{
    lexer::{Block, Keyword},
    parser::definition::VariableAssignment,
};

use super::{
    definition::{Expression, Statement, StatementKind, VariableDeclaration},
    error::{ParserError, ParserErrorKind},
    parser::Parser,
    types::Type,
    util::get_block_identifier,
};

impl<'a> Parser<'a> {
    fn parse_assignment_value(&mut self) -> Result<Option<(Expression, &Block)>, ParserError> {
        if let Some(equals) = self.get(&[Keyword::Equals]) {
            Ok(Some((self.expression()?, equals)))
        } else {
            Ok(None)
        }
    }

    fn new_assignment(&mut self) -> Result<Statement, ParserError> {
        if let Some(left) = self.get(&[Keyword::Identifier]) {
            let assignment_type: Option<Type> = self
                .get(&[Keyword::Colon])
                .map(|_| self.parse_type())
                .transpose()?;

            let Some((right, _)) = self.parse_assignment_value()? else {
                return Err(ParserError::new(
                    self.peek_or_eof()?.pos.clone(),
                    ParserErrorKind::Expected(&[Keyword::Equals]),
                ));
            };

            let Some(identifier) = get_block_identifier(&left) else {
                return Err(ParserError::new(
                    left.pos.clone(),
                    ParserErrorKind::Expected(&[Keyword::Identifier]),
                ));
            };

            return Ok(Statement {
                pos: left.pos.start..right.pos.end,
                kind: StatementKind::VariableDeclaration(VariableDeclaration {
                    typ: assignment_type,
                    identifier,
                    identifier_pos: left.pos.clone(),
                    right_pos: right.pos.clone(),
                    right: Box::new(right),
                }),
            });
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

        if let Some((right, equals)) = self.parse_assignment_value()? {
            // let identifier = match get_expr_identifier(&left) {
            //     Some(identifier) => identifier,
            //     _ => {
            //         return Err(ParserError::new(
            //             left.pos.clone(),
            //             ParserErrorKind::AssignmentToNonIdentifier {
            //                 equals_pos: equals.pos.clone(),
            //             },
            //         ))
            //     }
            // };

            return Ok(Statement {
                pos: left.pos.start..right.pos.end,
                kind: StatementKind::VariableAssignment(VariableAssignment {
                    left_pos: left.pos.clone(),
                    left: Box::new(left),
                    right_pos: right.pos.clone(),
                    right: Box::new(right),
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
