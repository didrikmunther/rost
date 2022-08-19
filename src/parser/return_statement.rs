use crate::lexer::Keyword;

use super::{
    definition::{Return, Statement, StatementKind},
    error::ParserError,
    parser::Parser,
};

impl<'a> Parser<'a> {
    pub fn return_statement(&mut self) -> Result<Statement, ParserError> {
        if let Some(ret) = self.get(&[Keyword::Return]) {
            let expr = self.expression()?;

            return Ok(Statement {
                pos: ret.pos.start..expr.pos.end,
                kind: StatementKind::Return(Return {
                    value: Box::new(expr),
                }),
            });
        }

        self.assignment()
    }
}
