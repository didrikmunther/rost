use super::{Letter, Token, Literal};

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

pub fn literal_number_lexer<'a>(chars: &'a [Letter]) -> Option<(Token, &'a [Letter])> {
    let mut buf = Vec::<char>::new();

    for (i, &(_pos, cur, eof)) in chars.into_iter().enumerate() {
        if buf.is_empty() && cur.is_whitespace() {
            continue;
        }

        if !cur.is_numeric() || cur.is_whitespace() || eof {
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