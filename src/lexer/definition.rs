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
    Ampersand,
    Slash,
    Arrow,
    ParLeft,
    ParRight,
    BraceLeft,
    BraceRight,
    BracketLeft,
    BracketRight,
    Comma,
    Colon,
    LessThan,
    GreaterThan,
    Equality,
    Dot,

    // Types
    Int,
    Bool,
    String,
    Char,
    Pointer,
    Struct,

    // Abstract keywords
    Eof,
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
    Eof,
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub pos: Range<usize>,
    pub token: Token,
    pub kind: Keyword,
}
