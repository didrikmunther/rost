use super::{Letter, Lexer, LexerError, LexerErrorKind, Literal, Token};

pub struct StringLexer;

fn get_escaped(c: char) -> Result<char, LexerErrorKind> {
    Ok(match c {
        'n' => '\n',
        't' => '\t',
        '\\' => '\\',
        _ => return Err(LexerErrorKind::UnknownEscapeSequence(c)),
    })
}

impl Lexer for StringLexer {
    fn lex(&self, chars: &[Letter]) -> Result<Option<(Token, usize)>, LexerError> {
        let mut buf = Vec::<char>::new();
        let mut is_string = false;
        let mut start = 0;
        let mut escaped = false;

        for (i, &(pos, cur, eof)) in chars.iter().enumerate() {
            if !is_string && cur.is_whitespace() {
                continue;
            }

            if !is_string && cur != '"' {
                return Ok(None);
            }

            if is_string && eof {
                return Err(LexerError::new(
                    start..start + 1,
                    LexerErrorKind::UnterminatedQuote,
                ));
            }

            if is_string && cur == '"' {
                return Ok(Some((
                    Token::Literal(Literal::String(buf.iter().collect())),
                    i + 1,
                )));
            }

            if !is_string && buf.is_empty() && cur == '"' {
                start = pos;
                is_string = true;
                continue;
            }

            if escaped {
                buf.push(match get_escaped(cur) {
                    Ok(c) => c,
                    Err(err) => return Err(LexerError::new(start + i - 1..start + i + 1, err)),
                });
                escaped = false;
            } else {
                escaped = cur == '\\';
                if !escaped {
                    buf.push(cur);
                }
            }
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{error::LexerErrorKind, letter::get_letters};

    #[test]
    fn string_works() {
        let letters = &get_letters("\"hej\"");
        let lexed = StringLexer.lex(letters);

        assert_eq!(
            lexed,
            Ok(Some((
                Token::Literal(Literal::String(String::from("hej"))),
                5
            )))
        );
    }

    #[test]
    fn eof_err_works() {
        let letters = &get_letters("\"hej");
        let lexed = StringLexer.lex(letters);

        assert_eq!(
            lexed,
            Err(LexerError::new(0..1, LexerErrorKind::UnterminatedQuote))
        );
    }
}
