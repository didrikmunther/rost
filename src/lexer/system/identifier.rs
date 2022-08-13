use super::{Letter, Lexer, Token};

fn is_identifier(word: &str) -> bool {
    let mut chars = word.chars();
    chars.next().map(char::is_alphabetic).unwrap_or(false) && chars.all(char::is_alphanumeric)
}

pub struct IdentifierLexer;

impl IdentifierLexer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Lexer for IdentifierLexer {
    fn lex<'a>(&self, chars: &'a [Letter]) -> Option<(Token, &'a [Letter])> {
        let mut buf = Vec::<char>::new();

        for (i, &(_pos, cur, eof)) in chars.into_iter().enumerate() {
            if buf.is_empty() && cur.is_whitespace() {
                continue;
            }

            if !cur.is_alphanumeric() || cur.is_whitespace() || eof {
                let word: String = buf.iter().collect();

                if word.len() <= 0 || !is_identifier(&word) {
                    return None;
                }

                return Some((Token::Identifier(word), &chars[i..]));
            }

            buf.push(cur);
        }

        None
    }
}
