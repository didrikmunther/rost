use super::{Letter, Lexer, LexerError, Token};

fn is_identifier(word: &str) -> bool {
    let mut chars = word.chars();
    chars
        .next()
        .map(|c| c.is_alphabetic() || c == '_')
        .unwrap_or(false)
        && chars.all(|c| c.is_alphanumeric() || c == '_')
}

pub struct IdentifierLexer;

impl IdentifierLexer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Lexer for IdentifierLexer {
    fn lex(&self, chars: &[Letter]) -> Result<Option<(Token, usize)>, LexerError> {
        let mut buf = Vec::<char>::new();

        for (i, &(_pos, cur, eof)) in chars.iter().enumerate() {
            if buf.is_empty() && cur.is_whitespace() {
                continue;
            }

            if !(cur.is_alphanumeric() || cur == '_') || cur.is_whitespace() || eof {
                let word: String = buf.iter().collect();

                if word.is_empty() || !is_identifier(&word) {
                    return Ok(None);
                }

                return Ok(Some((Token::Identifier(word), i)));
            }

            buf.push(cur);
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::letter::get_letters;

    #[test]
    fn is_identifier_works() {
        assert!(is_identifier("a"));
        assert!(is_identifier("abc"));
        assert!(is_identifier("abc1"));
        assert!(!is_identifier("1abc"));
        assert!(!is_identifier("+abc"));
        assert!(!is_identifier(""));
    }

    #[test]
    fn identifier_works() {
        let letters = &get_letters("abc");
        let lexed = IdentifierLexer::new().lex(letters);

        assert_eq!(lexed, Ok(Some((Token::Identifier(String::from("abc")), 3))));
    }
}
