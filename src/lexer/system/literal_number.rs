use super::{Letter, Lexer, Literal, Token};

fn is_number(word: &str) -> bool {
    word.chars().all(char::is_numeric)
}

fn get_literal(word: &str) -> Option<Literal> {
    match word {
        "true" => return Some(Literal::Bool(true)),
        "false" => return Some(Literal::Bool(false)),
        _ => {}
    };

    if is_number(word) {
        if let Some(number) = word.parse::<i32>().ok() {
            return Some(Literal::Int(number));
        }
    }

    None
}

pub struct LiteralNumberLexer;

impl LiteralNumberLexer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Lexer for LiteralNumberLexer {
    fn lex<'a>(&self, chars: &'a [Letter]) -> Option<(Token, &'a [Letter])> {
        let mut buf = Vec::<char>::new();

        for (i, &(_pos, cur, eof)) in chars.into_iter().enumerate() {
            if buf.is_empty() && cur.is_whitespace() {
                continue;
            }

            if !cur.is_digit(10) || cur.is_whitespace() || eof {
                let word: String = buf.iter().collect();

                if word.len() <= 0 {
                    return None;
                }

                if let Some(literal) = get_literal(&word) {
                    return Some((Token::Literal(literal), &chars[i..]));
                } else {
                    return None;
                }
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
    fn literal_number_works() {
        let letters = &get_letters("5");
        let lexed = LiteralNumberLexer::new().lex(letters);
        let rest: &[Letter] = &[EOF];

        assert_eq!(lexed, Some((Token::Literal(Literal::Int(5)), rest)));
    }
}