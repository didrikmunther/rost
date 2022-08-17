use std::ops::Range;

use crate::lexer::{Keyword, Literal};

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
pub struct Assignment {
    pub is_new: bool,
    pub identifier: String,
    pub identifier_pos: Range<usize>,
    pub value: Box<Expression>,
    pub value_pos: Range<usize>,
}

#[derive(Debug)]
pub enum StatementKind {
    Expression(Expression),
    Assignment(Assignment)
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
