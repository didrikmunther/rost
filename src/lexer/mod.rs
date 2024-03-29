pub use definition::{Block, Keyword, Literal, Token};
pub use error::{LexerError, LexerErrorKind};
pub use letter::Letter;

use self::system::LiteralBoolLexer;
use letter::{get_letters, UnexpectedToken};
use system::{
    CommentLexer, IdentifierLexer, KeywordLexer, Lexer, LiteralNumberLexer, StringLexer,
    SymbolLexer,
};

pub mod definition;

mod error;
mod letter;
mod system;

pub fn lex(text: &str) -> Result<Vec<Block>, LexerError> {
    let mut res = Vec::<Block>::new();
    let mut chars: &[Letter] = &get_letters(text);
    let mut pos = 0;

    let lexers: Vec<Box<dyn Lexer>> = vec![
        Box::new(CommentLexer),
        Box::new(StringLexer),
        Box::new(KeywordLexer::new()),
        Box::new(LiteralNumberLexer),
        Box::new(LiteralBoolLexer),
        Box::new(IdentifierLexer),
        Box::new(SymbolLexer),
    ];

    loop {
        if chars.is_empty() {
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
                let kind = match token {
                    Token::Identifier(_) => Keyword::Identifier,
                    Token::Keyword(keyword) => keyword,
                    Token::Eof => Keyword::Eof,
                    Token::Literal(_) => Keyword::Literal,
                    Token::Comment(_) => Keyword::Comment,
                };

                // Don't add comments to token list, might change in future
                if kind != Keyword::Comment {
                    res.push(Block {
                        pos: pos..pos + new_pos,
                        token,
                        kind,
                    });
                }

                chars = &chars[new_pos..];
                pos += new_pos;
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
        token: Token::Eof,
        kind: Keyword::Eof,
    });

    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::lexer::error::LexerErrorKind;

    use super::*;
    use Token::*;

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
                    pos: 13..16,
                    token: Keyword(super::Keyword::Let),
                    kind: super::Keyword::Let,
                },
                Block {
                    pos: 17..18,
                    token: Identifier(String::from("a")),
                    kind: super::Keyword::Identifier,
                },
                Block {
                    pos: 19..20,
                    token: Keyword(super::Keyword::Equals),
                    kind: super::Keyword::Equals,
                },
                Block {
                    pos: 21..22,
                    token: Literal(super::Literal::Int(5)),
                    kind: super::Keyword::Literal,
                },
                Block {
                    pos: 22..23,
                    token: Keyword(super::Keyword::Semicolon),
                    kind: super::Keyword::Semicolon,
                },
                Block {
                    pos: 36..39,
                    token: Keyword(super::Keyword::Let),
                    kind: super::Keyword::Let,
                },
                Block {
                    pos: 44..45,
                    token: Identifier(String::from("b")),
                    kind: super::Keyword::Identifier,
                },
                Block {
                    pos: 45..46,
                    token: Keyword(super::Keyword::Equals),
                    kind: super::Keyword::Equals,
                },
                Block {
                    pos: 46..51,
                    token: Literal(super::Literal::String(String::from("abc"))),
                    kind: super::Keyword::Literal,
                },
                Block {
                    pos: 52..53,
                    token: Keyword(super::Keyword::Semicolon),
                    kind: super::Keyword::Semicolon,
                },
                Block {
                    pos: 63..63,
                    token: Eof,
                    kind: super::Keyword::Eof,
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
            Err(LexerError::new(
                17..18,
                LexerErrorKind::UnexpectedToken('¢')
            ))
        );
    }
}
