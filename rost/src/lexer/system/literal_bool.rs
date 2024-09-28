use super::{Letter, Lexer, LexerError, Literal, Token};

fn get_literal_bool(word: &str) -> Option<Literal> {
    match word {
        "true" => Some(Literal::Bool(true)),
        "false" => Some(Literal::Bool(false)),
        _ => None,
    }
}

pub struct LiteralBoolLexer;

impl Lexer for LiteralBoolLexer {
    fn lex(&self, chars: &[Letter]) -> Result<Option<(Token, usize)>, LexerError> {
        let mut buf = Vec::<char>::new();

        for (i, &(_pos, cur, eof)) in chars.iter().enumerate() {
            if buf.is_empty() && cur.is_whitespace() {
                continue;
            }

            if cur.is_whitespace() || eof {
                let word: String = buf.iter().collect();

                if word.is_empty() {
                    return Ok(None);
                }

                if let Some(literal) = get_literal_bool(&word) {
                    return Ok(Some((Token::Literal(literal), i)));
                } else {
                    return Ok(None);
                }
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
    fn literal_bool_works() {
        let letters = &get_letters("true");
        let lexed = LiteralBoolLexer.lex(letters);

        assert_eq!(lexed, Ok(Some((Token::Literal(Literal::Bool(true)), 4))));
    }
}
