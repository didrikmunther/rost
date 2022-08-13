use std::iter;

mod system;
use system::{IdentifierLexer, KeywordLexer, Lexer, LiteralNumberLexer, SymbolLexer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Keyword {
    Let,
    Equals,
    Semicolon,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    Literal(Literal),
}

pub type Letter = (usize, char, bool);

pub fn lex(text: &str) -> Result<Vec<Token>, String> {
    let mut res = Vec::<Token>::new();
    let chars: Vec<Letter> = text
        .chars()
        .enumerate()
        .map(|(i, v)| (i, v, false))
        .chain(iter::once((0, ' ', true)))
        .collect();

    let mut chars: &[Letter] = &chars;

    let lexers: Vec<Box<dyn Lexer>> = vec![
        Box::new(KeywordLexer::new()),
        Box::new(IdentifierLexer::new()),
        Box::new(LiteralNumberLexer::new()),
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
            if let Some((token, new_chars)) = lexer.lex(chars) {
                // println!("New token: {:?}", token);
                res.push(token);
                chars = new_chars;
                hit = true;
                break;
            }
        }

        if !hit {
            return Err(format!("Unexpected token: {:?}", chars.iter().next()));
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
            let     b=1 ;
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
                Token::Literal(Literal::Int(1)),
                Token::Keyword(Keyword::Semicolon)
            ])
        );
    }
}
