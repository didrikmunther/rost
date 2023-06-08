use super::{Letter, Lexer, LexerError, Literal, Token};

fn is_number(word: &str) -> bool {
    word.chars().all(char::is_numeric)
}

fn get_literal_num(word: &str) -> Option<Literal> {
    if is_number(word) {
        if let Ok(number) = word.parse::<i32>() {
            return Some(Literal::Int(number));
        }
    }

    None
}

pub struct LiteralNumberLexer;

impl Lexer for LiteralNumberLexer {
    fn lex(&self, chars: &[Letter]) -> Result<Option<(Token, usize)>, LexerError> {
        let mut buf = Vec::<char>::new();

        for (i, &(_pos, cur, eof)) in chars.iter().enumerate() {
            if buf.is_empty() && cur.is_whitespace() {
                continue;
            }

            if !cur.is_ascii_digit() || cur.is_whitespace() || eof {
                let word: String = buf.iter().collect();

                if word.is_empty() {
                    return Ok(None);
                }

                if let Some(literal) = get_literal_num(&word) {
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
    fn is_number_works() {
        assert!(is_number("1"));
        assert!(is_number("123"));
        assert!(!is_number("a"));
        assert!(!is_number("a123"));
    }

    #[test]
    fn literal_number_works() {
        let letters = &get_letters("5");
        let lexed = LiteralNumberLexer.lex(letters);

        assert_eq!(lexed, Ok(Some((Token::Literal(Literal::Int(5)), 1))));
    }
}
