use std::ops::Range;

use crate::{
    lexer::{Block, Keyword, Token},
    parser_todo,
};

use super::{
    definition::{Declaration, Expression, ExpressionKind, Primary},
    error::ParserError,
    Parser,
};

pub fn get_expr_identifier(expr: &Expression) -> Option<String> {
    match expr.kind {
        ExpressionKind::Primary(Primary::Identifier(ref identifier)) => Some(identifier.clone()),
        _ => None,
    }
}

pub fn get_block_identifier(block: &Block) -> Option<String> {
    match &block.token {
        Token::Identifier(identifier) => Some(identifier.clone()),
        _ => None,
    }
}

impl<'a> Parser<'a> {
    pub fn get_body(&mut self) -> Result<(Vec<Declaration>, Range<usize>), ParserError> {
        if let Some(open) = self.get(&[Keyword::BracketLeft]) {
            let mut content: Vec<Declaration> = Vec::new();

            loop {
                if self.is_end() {
                    return parser_todo!(open.pos.clone(), "error");
                }

                if let Some(close) = self.get(&[Keyword::BracketRight]) {
                    return Ok((content, open.pos.start..close.pos.end));
                }

                content.push(self.declaration()?);
            }
        } else {
            let declaration = self.declaration()?;
            let pos = declaration.pos.clone();
            
            Ok((vec![declaration], pos))
        }
    }
}
