use crate::lexer::{Block, Keyword};

use self::{
    definition::{Ast, Declaration, Expression, Statement},
    error::{ParserError, ParserErrorKind},
};

pub mod definition;
pub mod types;
pub mod util;

mod addition;
mod assignment;
mod comparison;
mod error;
mod function_call;
mod function_declaration;
mod if_statement;
mod array_index;
mod member;
mod multiplication;
mod parenthesis;
mod primary;
mod reference;
mod return_statement;
mod struct_construction;
mod struct_declaration;
mod unexpected;
mod while_statement;

pub fn parse(document: &Vec<Block>) -> Result<Ast, ParserError> {
    Parser::new(document).parse()
}

pub struct Parser<'a> {
    pub index: usize,
    document: &'a Vec<Block>,
}

impl<'a> Parser<'a> {
    pub fn new(document: &'a Vec<Block>) -> Self {
        Self { index: 0, document }
    }

    pub fn parse(&mut self) -> Result<Ast, ParserError> {
        let mut program = vec![];

        while !self.is_end() {
            program.push(self.declaration()?);
        }

        Ok(program)
    }

    pub fn declaration(&mut self) -> Result<Declaration, ParserError> {
        self.struct_declaration()
    }

    pub fn statement(&mut self) -> Result<Statement, ParserError> {
        self.if_statement()
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

    pub fn peek_offset(&self, i: usize) -> Option<&'a Block> {
        self.get_at(self.index + i)
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
        self.check(Keyword::Eof).is_some()
    }

    fn check(&self, token: Keyword) -> Option<&'a Block> {
        if token != Keyword::Eof && self.is_end() {
            None
        } else {
            self.peek()
                .and_then(|v| if v.kind == token { Some(v) } else { None })
        }
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }

    pub fn advance_n(&mut self, i: usize) {
        self.index += i;
    }

    pub fn get(&mut self, tokens: &'static [Keyword]) -> Option<&'a Block> {
        for token in tokens {
            if let Some(block) = self.check(*token) {
                self.advance();
                return Some(block);
            }
        }

        None
    }

    pub fn get_peek(&mut self, tokens: &'static [Keyword]) -> Option<&'a Block> {
        for token in tokens {
            if let Some(block) = self.check(*token) {
                return Some(block);
            }
        }

        None
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
