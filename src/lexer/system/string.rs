use super::{Letter, Lexer, Literal, Token};

pub struct StringLexer;

impl StringLexer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Lexer for StringLexer {
    fn lex<'a>(&self, chars: &'a [Letter]) -> Option<(Token, &'a [Letter])> {
        let mut buf = Vec::<char>::new();
        let mut is_string = false;

        for (i, &(_pos, cur, eof)) in chars.into_iter().enumerate() {
            if buf.is_empty() && cur.is_whitespace() {
                continue;
            }

            if !is_string && cur != '"' {
                return None;
            }

            if is_string && eof {
                return None; // todo: this should return error
            }

            if is_string && cur == '"' {
                return Some((
                    Token::Literal(Literal::String(buf.iter().collect())),
                    &chars[i + 1..],
                ));
            }

            if !is_string && buf.is_empty() && cur == '"' {
                is_string = true;
                continue;
            }

            buf.push(cur);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::letter::{get_letters, EOF};

    #[test]
    fn string_works() {
        let letters = &get_letters("\"hej\"");
        let lexed = StringLexer::new().lex(letters);
        let rest: &[Letter] = &[EOF];

        assert_eq!(
            lexed,
            Some((Token::Literal(Literal::String(String::from("hej"))), rest))
        );
    }
}
