use crate::lexer::Keyword;

use super::{
    definition::{Declaration, IfStatement, Statement, StatementKind},
    error::ParserError,
    parser::Parser,
};

impl<'a> Parser<'a> {
    pub fn if_statement(&mut self) -> Result<Statement, ParserError> {
        if let Some(if_block) = self.get(&[Keyword::If]) {
            let expr = self.expression()?;
            if let None = self.get(&[Keyword::BracketLeft]) {
                todo!("error")
            }

            let mut content: Vec<Declaration> = Vec::new();

            loop {
                if self.is_end() {
                    todo!("error")
                }

                if let Some(bracket_right) = self.get(&[Keyword::BracketRight]) {
                    return Ok(Statement {
                        pos: if_block.pos.start..bracket_right.pos.end,
                        kind: StatementKind::IfStatement(IfStatement {
                            condition: Box::new(expr),
                            content,
                            elses: Vec::new(),
                        }),
                    });
                }

                content.push(self.declaration()?);
            }

            // while let Some(else_block) = self.get(&[Keyword::Else]) { }
        }

        self.return_statement()
    }
}
