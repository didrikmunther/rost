use std::ops::Range;

use crate::lexer::{Keyword, Literal};

use super::types::Type;

pub type AST = Vec<Declaration>;

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

#[derive(Debug, Clone)]
pub struct FunctionDeclarationParameter {
    pub identifier: String,
    pub typ: Type,
    pub pos: Range<usize>,
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub identifier: String,
    pub parameters: Vec<FunctionDeclarationParameter>,
    pub content: Vec<Declaration>,
    pub return_type: Option<Type>,
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
    IfStatements(Vec<IfStatement>),
    WhileStatement(WhileStatement),
    ReturnStatement(ReturnStatement),
}

#[derive(Debug)]
pub struct WhileStatement {
    pub condition: Box<Expression>,
    pub content: Vec<Declaration>,
}

#[derive(Debug)]
pub struct IfStatement {
    pub condition: Option<Box<Expression>>,
    pub content: Vec<Declaration>,
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub value: Box<Expression>,
}

#[derive(Debug)]
pub struct Expression {
    pub pos: Range<usize>,
    pub kind: ExpressionKind,
}

#[derive(Debug)]
pub enum ExpressionKind {
    Primary(Primary),
    Unary(Unary),
    Binary(Binary),
    FunctionCall(FunctionCall),
}

#[derive(Debug)]
pub struct FunctionCall {
    pub identifier: String,
    pub args: Vec<Box<Expression>>,
    pub identifier_pos: Range<usize>,
    pub parameters_pos: Range<usize>,
}

#[derive(Debug)]
pub struct Unary {
    pub expr: Box<Expression>,
    pub operator: Keyword,
    pub operator_pos: Range<usize>
}

#[derive(Debug)]
pub struct Binary {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: Keyword,
    pub operator_pos: Range<usize>
}

#[derive(Debug)]
pub enum Primary {
    Literal(Literal),
    Identifier(String),
}
