use std::ops::Range;

use crate::lexer::{Block, Keyword, Literal, Token};

use self::error::{ParserError, ParserErrorKind};

mod error;

pub type AST = Vec<Declaration>;

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
    Primary(Primary),
    Binary(Binary),
    FunctionCall(FunctionCall),
}

#[derive(Debug)]
pub struct FunctionCall {
    pub identifier: Box<Expression>,
    pub args: Vec<Box<Expression>>,
}

#[derive(Debug)]
pub struct Binary {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: Keyword,
}

#[derive(Debug)]
pub enum Primary {
    Literal(Literal),
    Identifier(String),
}

struct Parser<'a> {
    index: usize,
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

    fn declaration(&mut self) -> Result<Declaration, ParserError> {
        let statement = self.statement()?;

        Ok(Declaration {
            pos: statement.pos.clone(),
            kind: DeclarationKind::Statement(statement),
        })
    }

    fn statement(&mut self) -> Result<Statement, ParserError> {
        let expression = self.expression()?;

        Ok(Statement {
            pos: expression.pos.clone(),
            kind: StatementKind::Expression(expression),
        })
    }

    fn expression(&mut self) -> Result<Expression, ParserError> {
        self.addition()
    }

    fn addition(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.function_call()?;

        while let Some(block) = self.get(&[Keyword::Plus, Keyword::Minus]) {
            let right = self.primary()?;
            let pos = expr.pos.start..right.pos.end;

            expr = Expression {
                pos: pos.clone(),
                kind: ExpressionKind::Binary(Binary {
                    left: Box::new(expr),
                    right: Box::new(right),
                    operator: block.kind,
                }),
            }
        }

        Ok(expr)
    }

    fn function_call(&mut self) -> Result<Expression, ParserError> {
        let expr = self.primary()?;

        if let Some(open) = self.get(&[Keyword::ParLeft]) {
            let mut args = vec![];

            loop {
                if self.is_end() {
                    return Err(ParserError::new(
                        open.pos.clone(),
                        ParserErrorKind::UnterminatedParenthesis,
                    ));
                }

                if let None = self.get(&[Keyword::Comma]) {
                    if let Some(close) = self.get(&[Keyword::ParRight]) {
                        return Ok(Expression {
                            pos: open.pos.start..close.pos.start,
                            kind: ExpressionKind::FunctionCall(FunctionCall {
                                identifier: Box::new(expr),
                                args,
                            }),
                        });
                    }   
                }

                args.push(Box::new(self.expression()?));
            }
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expression, ParserError> {
        if let Some(block) = self.get(&[Keyword::Literal, Keyword::Identifier]) {
            return Ok(Expression {
                pos: block.pos.clone(),
                kind: ExpressionKind::Primary(match &block.token {
                    Token::Identifier(identifier) => Primary::Identifier(identifier.clone()),
                    Token::Literal(literal) => Primary::Literal(literal.clone()),
                    _ => {
                        return Err(ParserError::new(
                            block.pos.clone(),
                            ParserErrorKind::Unknown,
                        ))
                    }
                }),
            });
        }

        if let Some(block) = self.peek() {
            Err(ParserError::new(
                block.pos.clone(),
                ParserErrorKind::UnexpectedToken(format!("{:?}", block.token)),
            ))
        } else {
            return Err(ParserError::new(
                self.get_at(self.index - 1)
                    .map(|v| v.pos.clone())
                    .unwrap_or(0..0),
                ParserErrorKind::UnexpectedEOF,
            ));
        }
    }

    fn get_at(&self, index: usize) -> Option<&'a Block> {
        self.document.get(index)
    }

    fn peek(&self) -> Option<&'a Block> {
        self.get_at(self.index)
    }

    fn is_end(&self) -> bool {
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

    fn get(&mut self, tokens: &'static [Keyword]) -> Option<&'a Block> {
        for token in tokens {
            if let Some(block) = self.check(*token) {
                self.advance();
                return Some(block);
            }
        }

        return None;
    }
}

pub fn parse<'a>(document: &'a Vec<Block>) -> Result<AST, ParserError> {
    Parser::new(document).parse()
}
