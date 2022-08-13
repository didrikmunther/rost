use std::ops::Range;

mod system;
use system::{
    CommentLexer, IdentifierLexer, KeywordLexer, Lexer, LiteralNumberLexer, StringLexer,
    SymbolLexer,
};

mod letter;
pub use letter::Letter;
use letter::{get_letters, UnexpectedToken};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Keyword {
    Let,
    Equals,
    Semicolon,
    Plus,
    Minus,
    Arrow,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(String),
    Int(i32),
    Bool(bool),
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    Literal(Literal),
    Comment(String),
    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pos: Range<usize>,
    token: Token,
}

#[derive(Debug, PartialEq)]
pub struct LexerError {
    pos: Range<usize>,
    message: String,
}

pub fn lex(text: &str) -> Result<Vec<Block>, LexerError> {
    let mut res = Vec::<Block>::new();
    let mut chars: &[Letter] = &get_letters(text);
    let mut pos = 0;

    let lexers: Vec<Box<dyn Lexer>> = vec![
        Box::new(CommentLexer::new()),
        Box::new(StringLexer::new()),
        Box::new(KeywordLexer::new()),
        Box::new(LiteralNumberLexer::new()),
        Box::new(IdentifierLexer::new()),
        Box::new(SymbolLexer::new()),
    ];

    loop {
        if chars.len() <= 0 {
            break;
        }

        if chars[0].1.is_whitespace() {
            chars = &chars[1..];
            pos += 1;
            continue;
        }

        let mut hit = false;

        for lexer in &lexers {
            if let Some((token, new_pos)) = lexer.lex(chars)? {
                res.push(Block {
                    pos: pos..new_pos,
                    token,
                });

                chars = &chars[new_pos..];
                pos = new_pos;
                hit = true;
                break;
            }
        }

        if !hit {
            return Err(chars.iter().next().unwrap().unexpected_token());
        }
    }

    res.push(Block {
        pos: pos..pos,
        token: Token::EOF,
    });

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;
    use super::Keyword::*;

    #[test]
    fn lexer_works() {
        let lexed = lex("
            let a = 5;
            let     b=\"abc\" ;
        ");

        assert_eq!(
            lexed,
            Ok(vec![
                Block {
                    pos: 13..3,
                    token: Keyword(Let)
                },
                Block {
                    pos: 4..1,
                    token: Identifier(String::from("a"))
                },
                Block {
                    pos: 2..1,
                    token: Keyword(Equals)
                },
                Block {
                    pos: 2..1,
                    token: Literal(super::Literal::Int(5))
                },
                Block {
                    pos: 1..1,
                    token: Keyword(Semicolon)
                },
                Block {
                    pos: 14..3,
                    token: Keyword(Let)
                },
                Block {
                    pos: 8..1,
                    token: Identifier(String::from("b"))
                },
                Block {
                    pos: 1..1,
                    token: Keyword(Equals)
                },
                Block {
                    pos: 1..5,
                    token: Literal(super::Literal::String(String::from("abc")))
                },
                Block {
                    pos: 6..1,
                    token: Keyword(Semicolon)
                },
                Block {
                    pos: 11..11,
                    token: EOF
                }
            ])
        );
    }

    #[test]
    fn unexpected_token_err_works() {
        let lexed = lex("
            let ¢ = 5;
        ");

        assert_eq!(
            lexed,
            Err(LexerError {
                pos: 17..18,
                message: String::from("Unexpected token: '¢'")
            })
        );
    }
}
