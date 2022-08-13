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
}

#[derive(Debug, PartialEq)]
pub struct LexerError {
    pos: Range<usize>,
    message: String,
}

pub fn lex(text: &str) -> Result<Vec<Token>, LexerError> {
    let mut res = Vec::<Token>::new();
    let mut chars: &[Letter] = &get_letters(text);

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
            continue;
        }

        let mut hit = false;

        for lexer in &lexers {
            if let Some((token, new_chars)) = lexer.lex(chars)? {
                res.push(token);
                chars = new_chars;
                hit = true;
                break;
            }
        }

        if !hit {
            return Err(chars.iter().next().unwrap().unexpected_token());
        }
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_works() {
        let lexed = lex("
            let a = 5;
            let     b=\"abc\" ;
        ");

        assert_eq!(
            lexed,
            Ok(vec![
                Token::Keyword(Keyword::Let),
                Token::Identifier(String::from("a")),
                Token::Keyword(Keyword::Equals),
                Token::Literal(Literal::Int(5)),
                Token::Keyword(Keyword::Semicolon),
                Token::Keyword(Keyword::Let),
                Token::Identifier(String::from("b")),
                Token::Keyword(Keyword::Equals),
                Token::Literal(Literal::String(String::from("abc"))),
                Token::Keyword(Keyword::Semicolon)
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
