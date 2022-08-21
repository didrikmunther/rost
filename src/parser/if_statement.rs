use std::ops::Range;

use crate::lexer::Keyword;

use super::{
    definition::{Declaration, IfStatement, Statement, StatementKind},
    error::ParserError,
    parser::Parser,
};

impl<'a> Parser<'a> {
    fn get_body(&mut self) -> Result<(Vec<Declaration>, Range<usize>), ParserError> {
        if let None = self.get(&[Keyword::BracketLeft]) {
            let declaration = self.declaration()?;
            let pos = declaration.pos.clone();
            return Ok((vec![declaration], pos));
        }

        let mut content: Vec<Declaration> = Vec::new();

        loop {
            if self.is_end() {
                todo!("error")
            }

            if let Some(bracket_right) = self.get(&[Keyword::BracketRight]) {
                return Ok((content, bracket_right.pos.clone()));
            }

            content.push(self.declaration()?);
        }
    }

    pub fn if_statement(&mut self) -> Result<Statement, ParserError> {
        if let Some(if_block) = self.get(&[Keyword::If]) {
            let mut statements = Vec::new();

            let condition = self.expression()?;
            let (content, content_end) = self.get_body()?;

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
                pos: if_block.pos.start..content_end.end,
                kind: StatementKind::IfStatements(statements),
            });
        }

        self.return_statement()
    }
}
