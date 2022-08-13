use std::ops::Range;

use crate::lexer::{Literal, Token, Block};

#[derive(Debug)]
pub struct ParserError;

pub type Program = Vec<Declaration>;

#[derive(Debug)]
pub struct Declaration {
    pub pos: Range<usize>,
    pub kind: DeclarationKind,
}

#[derive(Debug)]
pub enum DeclarationKind {
    Statement(Statement),
}

#[derive(Debug)]
pub struct Statement {
    pub pos: Range<usize>,
    pub kind: StatementKind,
}

#[derive(Debug)]
pub enum StatementKind {
    Expression(Expression),
}

#[derive(Debug)]
pub struct Expression {
    pub pos: Range<usize>,
    pub kind: ExpressionKind,
}

#[derive(Debug)]
pub enum ExpressionKind {
    Empty,
    Primary(Primary),
    Binary(BinaryExpression),
}

#[derive(Debug)]
pub struct BinaryExpression {
    pos: Range<usize>,
    left: Box<Expression>,
    right: Box<Expression>,
    operator: Token,
}

#[derive(Debug)]
pub enum Primary {
    Literal(Literal),
    Identifier(String),
}

struct Parser {
    index: usize,
    document: Vec<Block>,
}

impl Parser {
    pub fn new(document: Vec<Block>) -> Self {
        Self { index: 0, document }
    }

    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut program = vec![];

        return Ok(program);
    }

	// fn get_at(&self, index: usize) -> Option<&'a Block> {
    //     self.document.get(index)
    //         .map(|v| *v)
    // }
 
    // fn peek(&self) -> Option<&'a Block> {
    //     self.get_at(self.index + 1)
    // }

    // fn reverse(&mut self, index: usize) {
    //     self.index = index;
    // }

    // fn is_end(&self) -> bool {
    //     self.check(Token::EOF).is_some()
    // }

	// fn check(&self, token: Token) -> Option<&'a Block> {
    //     if token != Token::EOF && self.is_end() {
    //         None
    //     } else {
    //         self.peek()
    //             .and_then(|v| if v.token == token {
    //                 Some(v)
    //             } else {
    //                 None
    //             })
    //     }
    // }

	// fn get(&mut self, tokens: &'static [Token]) -> Option<Token> {
    //     for token in tokens {
    //         if let Some(block) = self.check(*token) {
    //             self.advance();
    //             return Some(block);
    //         }
    //     }

    //     return None;
    // }
}

pub fn parse<'a>(document: Vec<Block>, _text: &'a str) -> Result<Program, ParserError> {
    Parser::new(document).parse()
}
