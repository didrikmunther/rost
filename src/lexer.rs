use phf::phf_map;
use std::iter;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Keyword {
    Let,
    Equals,
    Semicolon,
}

static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "let" => Keyword::Let,
};

// Symbols are not alphanumerical
static SYMBOLS: phf::Map<&'static str, Keyword> = phf_map! {
    "=" => Keyword::Equals,
    ";" => Keyword::Semicolon,
    "=>" => Keyword::Equals // todo: temp
};

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

fn is_identifier(word: &str) -> bool {
    let mut chars = word.chars();
    chars.next().map(char::is_alphabetic).unwrap_or(false) && chars.all(char::is_alphanumeric)
}

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

pub fn lex(text: &str) -> Result<Vec<Token>, String> {
    let mut res = Vec::<Token>::new();
    let mut buf = Vec::<char>::new();
    let chars = text.chars().enumerate().map(|(i, v)| (i, v, false));

    for (i, cur, eof) in chars.chain(iter::once((0, ' ', true))) {
        if buf.is_empty() && cur.is_whitespace() {
            continue;
        }

        println!("{:?}", buf);
        println!("{:?}", (i, cur));

        let word: String = buf.iter().collect();
        let identifier = is_identifier(&word);

        let eow = cur.is_whitespace() || !cur.is_alphanumeric(); // End of word
        let eos = !identifier && (cur.is_whitespace() || cur.is_alphanumeric()); // End of symbol

        if word.len() > 0 && (eof || eow || eos) {
            if let Some(literal) = get_literal(&word) {
                res.push(Token::Literal(literal));
            } else if identifier {
                if let Some(keyword) = KEYWORDS.get(&word) {
                    res.push(Token::Keyword(*keyword));
                } else {
                    res.push(Token::Identifier(word));
                }
            } else if let Some(symbol) = SYMBOLS.get(&word) {
                res.push(Token::Keyword(*symbol));
            } else {
                return Err(format!("Not recognized: {}", word));
            }

            buf.clear();

            if !cur.is_whitespace() {
                buf.push(cur);
            }

            continue;
        }

        buf.push(cur);
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_works() {
        assert_eq!(
            lex("
				let ab = 5 ; let k = true;
			"),
            Ok(vec![
                Token::Keyword(Keyword::Let),
                Token::Identifier("ab".to_string()),
                Token::Keyword(Keyword::Equals),
                Token::Literal(Literal::Int(5)),
                Token::Keyword(Keyword::Semicolon),
                Token::Keyword(Keyword::Let),
                Token::Identifier("k".to_string()),
                Token::Keyword(Keyword::Equals),
                Token::Literal(Literal::Bool(true)),
                Token::Keyword(Keyword::Semicolon),
            ])
        );
    }
}
