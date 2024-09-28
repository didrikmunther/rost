use crate::lexer::Keyword;

use super::{
    definition::{Statement, StatementKind, WhileStatement},
    error::{ParserError, ParserErrorKind},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn while_statement(&mut self) -> Result<Statement, ParserError> {
        if let Some(if_block) = self.get(&[Keyword::While]) {
            let condition = self.expression()?;
            let (content, size) = self.get_body()?;

            return Ok(Statement {
                pos: if_block.pos.start..size.end,
                kind: StatementKind::WhileStatement(WhileStatement {
                    condition: Box::new(condition),
                    content,
                }),
            });
        }

        let statement = self.return_statement()?;

        if self.get(&[Keyword::Semicolon]).is_none() {
            return Err(ParserError::new(
                statement.pos,
                ParserErrorKind::ExpectedSemicolon,
            ));
        }

        Ok(statement)
    }
}
