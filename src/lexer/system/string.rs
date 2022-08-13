use super::{Letter, Lexer, LexerError, Literal, Token};

pub struct StringLexer;

impl StringLexer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Lexer for StringLexer {
    fn lex<'a>(&self, chars: &'a [Letter]) -> Result<Option<(Token, &'a [Letter])>, LexerError> {
        let mut buf = Vec::<char>::new();
        let mut is_string = false;
        let mut start = 0;
        let mut last = 0;

        for (i, &(pos, cur, eof)) in chars.into_iter().enumerate() {
            if buf.is_empty() && cur.is_whitespace() {
                continue;
            }

            if !is_string && cur != '"' {
                return Ok(None);
            }

            if is_string && eof {
                return Err(LexerError {
                    pos: start..last,
                    message: String::from("Unexpected EOF for string"),
                });
            }

            if is_string && cur == '"' {
                return Ok(Some((
                    Token::Literal(Literal::String(buf.iter().collect())),
                    &chars[i + 1..],
                )));
            }

            if !is_string && buf.is_empty() && cur == '"' {
                start = pos;
                is_string = true;
                continue;
            }

            buf.push(cur);
            last = pos;
        }

        Ok(None)
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
            Ok(Some((
                Token::Literal(Literal::String(String::from("hej"))),
                rest
            )))
        );
    }

    #[test]
    fn eof_err_works() {
        let letters = &get_letters("\"hej");
        let lexed = StringLexer::new().lex(letters);

        assert_eq!(
            lexed,
            Err(LexerError {
                pos: 0..3,
                message: String::from("Unexpected EOF for string")
            })
        );
    }
}
