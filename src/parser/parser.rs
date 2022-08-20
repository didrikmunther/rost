use crate::lexer::{Block, Keyword};

use super::{
    definition::{Declaration, Expression, Statement},
    error::{ParserError, ParserErrorKind},
    AST,
};

pub struct Parser<'a> {
    pub index: usize,
    document: &'a Vec<Block>,
}

impl<'a> Parser<'a> {
    pub fn new(document: &'a Vec<Block>) -> Self {
        Self { index: 0, document }
    }

    pub fn parse(&mut self) -> Result<AST, ParserError> {
        let mut program = vec![];

        while !self.is_end() {
            program.push(self.declaration()?);
        }

        return Ok(program);
    }

    pub fn declaration(&mut self) -> Result<Declaration, ParserError> {
        self.function_declaration()
    }

    pub fn statement(&mut self) -> Result<Statement, ParserError> {
        self.return_statement()
    }

    pub fn expression(&mut self) -> Result<Expression, ParserError> {
        self.comparison()
    }

    pub fn get_at(&self, index: usize) -> Option<&'a Block> {
        self.document.get(index)
    }

    pub fn peek(&self) -> Option<&'a Block> {
        self.get_at(self.index)
    }

    pub fn peek_or_eof(&self) -> Result<&'a Block, ParserError> {
        match self.peek() {
            Some(block) => Ok(block),
            None => Err(ParserError::new(
                self.document.last().unwrap().pos.clone(),
                ParserErrorKind::UnexpectedEOF,
            )),
        }
    }

    pub fn is_end(&self) -> bool {
        self.check(Keyword::EOF).is_some()
    }

    fn check(&self, token: Keyword) -> Option<&'a Block> {
        if token != Keyword::EOF && self.is_end() {
            None
        } else {
            self.peek()
                .and_then(|v| if v.kind == token { Some(v) } else { None })
        }
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    pub fn get(&mut self, tokens: &'static [Keyword]) -> Option<&'a Block> {
        for token in tokens {
            if let Some(block) = self.check(*token) {
                self.advance();
                return Some(block);
            }
        }

        return None;
    }

    pub fn expect(&mut self, tokens: &'static [Keyword]) -> Result<&'a Block, ParserError> {
        for token in tokens {
            if let Some(block) = self.check(*token) {
                self.advance();
                return Ok(block);
            }
        }

        return Err(ParserError::new(
            self.peek_or_eof()?.pos.clone(),
            ParserErrorKind::Expected(tokens),
        ));
    }
}
