use std::ops::Range;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Keyword {
    Let,
    Fn,
    Return,
    Equals,
    Semicolon,
    If,
    Else,
    While,
    Plus,
    Minus,
    Asterix,
    Slash,
    Arrow,
    ParLeft,
    ParRight,
    BracketLeft,
    BracketRight,
    Comma,
    Colon,
    LessThan,
    GreaterThan,
    Equality,

    // Temp
    Null,
    Binary,

    // Types
    Int,
    Bool,
    String,

    // Abstract keywords
    EOF,
    Identifier,
    Literal,
    Comment,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Int(i32),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    Literal(Literal),
    Comment(String),
    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub pos: Range<usize>,
    pub token: Token,
    pub kind: Keyword,
}