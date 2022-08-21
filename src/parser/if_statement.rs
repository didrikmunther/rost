use crate::lexer::Keyword;

use super::{
    definition::{IfStatement, Statement, StatementKind},
    error::ParserError,
    parser::Parser,
};

impl<'a> Parser<'a> {
    pub fn if_statement(&mut self) -> Result<Statement, ParserError> {
        if let Some(if_block) = self.get(&[Keyword::If]) {
            let mut statements = Vec::new();

            let condition = self.expression()?;
            let (content, size) = self.get_body()?;

            statements.push(IfStatement {
                condition: Some(Box::new(condition)),
                content,
            });

            while let Some(_) = self.get(&[Keyword::Else]) {
                if let Some(_) = self.get(&[Keyword::If]) {
                    statements.push(IfStatement {
                        condition: Some(Box::new(self.expression()?)),
                        content: self.get_body()?.0,
                    });
                } else {
                    statements.push(IfStatement {
                        condition: None,
                        content: self.get_body()?.0,
                    });

                    break;
                }
            }

            return Ok(Statement {
                pos: if_block.pos.start..size.end,
                kind: StatementKind::IfStatements(statements),
            });
        }

        self.while_statement()
    }
}
