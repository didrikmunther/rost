use crate::lexer::Keyword;

use super::{
    definition::{Statement, StatementKind, Assignment, Expression, ExpressionKind, Primary},
    error::ParserError,
    parser::Parser,
};

fn get_identifier(expr: &Expression) -> Result<String, ParserError> {
    match expr.kind {
        ExpressionKind::Primary(ref primary) => match primary {
            Primary::Identifier(identifier) => Ok(identifier.clone()),
            _ => todo!()
        },
        _ => todo!()
    }
}

impl<'a> Parser<'a> {
    pub fn assignment(&mut self) -> Result<Statement, ParserError> {
        let is_new = self.get(&[Keyword::Let]).is_some(); // This consumes a possible let keyword
        let left = self.expression()?;

        if let Some(_) = self.get(&[Keyword::Equals]) {
            let right = self.expression()?;

            return Ok(Statement {
                pos: left.pos.start..right.pos.end,
                kind: StatementKind::Assignment(Assignment {
                    is_new,
                    identifier: get_identifier(&left)?,
                    identifier_pos: left.pos.clone(),
                    value_pos: right.pos.clone(),
                    value: Box::new(right),
                })
            })
        }

        // No assignment, simply an expression
        Ok(Statement {
            pos: left.pos.clone(),
            kind: StatementKind::Expression(left),
        })
    }
}
