use std::ops::Range;

use crate::lexer::{Keyword, Literal};

pub type AST = Vec<Declaration>;
pub type Type = Keyword; // todo: this can also be custom made types

#[derive(Debug)]
pub struct Declaration {
    pub pos: Range<usize>,
    pub kind: DeclarationKind,
}

#[derive(Debug)]
pub enum DeclarationKind {
    Statement(Statement),
    FunctionDeclaration(FunctionDeclaration),
}

#[derive(Debug)]
pub struct FunctionDeclarationParameter {
    pub identifier: String,
    pub typ: Keyword,
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub identifier: String,
    pub parameters: Vec<FunctionDeclarationParameter>,
    pub content: Vec<Declaration>,
}

#[derive(Debug)]
pub struct Statement {
    pub pos: Range<usize>,
    pub kind: StatementKind,
}

#[derive(Debug)]
pub struct Assignment {
    pub is_new: bool,
    pub typ: Option<Type>,
    pub identifier: String,
    pub identifier_pos: Range<usize>,
    pub value: Box<Expression>,
    pub value_pos: Range<usize>,
}

#[derive(Debug)]
pub enum StatementKind {
    Expression(Expression),
    Assignment(Assignment),
    IfStatement(IfStatement),
    Return(Return),
}

#[derive(Debug)]
pub struct ElseStatement {
    pub condition: Box<Expression>,
    pub content: Vec<Declaration>,
}

#[derive(Debug)]
pub struct IfStatement {
    pub condition: Box<Expression>,
    pub content: Vec<Declaration>,
    pub elses: Vec<ElseStatement>,
}

#[derive(Debug)]
pub struct Return {
    pub value: Box<Expression>,
}

#[derive(Debug)]
pub struct Expression {
    pub typ: Type,
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
    pub identifier: String,
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
