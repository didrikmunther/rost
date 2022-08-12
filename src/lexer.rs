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

fn literal_number_parser<'a>(chars: &'a [Letter]) -> Option<(Token, &'a [Letter])> {
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

fn symbol_parser<'a>(chars: &'a [Letter]) -> Option<(Token, &'a [Letter])> {
    let mut buf = Vec::<char>::new();

    for (i, &(_pos, cur, eof)) in chars.into_iter().enumerate() {
        if buf.is_empty() && cur.is_whitespace() {
            continue;
        }

        if cur.is_alphanumeric() || cur.is_whitespace() || eof {
            let word: String = buf.iter().collect();

            if word.len() <= 0 {
                return None;
            }

            if let Some(&symbol) = SYMBOLS.get(&word) {
                return Some((Token::Keyword(symbol), &chars[i..]));
            } else {
                return None;
            }
        }

        buf.push(cur);
    }

    None
}

fn identifier_parser<'a>(chars: &'a [Letter]) -> Option<(Token, &'a [Letter])> {
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

fn keyword_parser<'a>(chars: &'a [Letter]) -> Option<(Token, &'a [Letter])> {
    match identifier_parser(chars) {
        Some((Token::Identifier(identifier), new_chars)) => {
            if let Some(keyword) = KEYWORDS.get(&identifier) {
                Some((Token::Keyword(*keyword), new_chars))
            } else {
                None
            }
        }
        _ => None,
    }
}

type Letter = (usize, char, bool);

pub fn lex(text: &str) -> Result<Vec<Token>, String> {
    let mut res = Vec::<Token>::new();
    let chars: Vec<Letter> = text
        .chars()
        .enumerate()
        .map(|(i, v)| (i, v, false))
        .chain(iter::once((0, ' ', true)))
        .collect();

    let mut chars: &[Letter] = &chars;

    let lexers = [
        literal_number_parser,
        keyword_parser,
        identifier_parser,
        symbol_parser,
    ];

    loop {
		if chars.len() <= 0 {
            break;
        }

        if chars[0].1.is_whitespace() {
            chars = &chars[1..];
            continue;
        }

        let mut hit = false;

        for lexer in lexers {
            if let Some((token, new_chars)) = lexer(chars) {
                println!("New token: {:?}", token);
                res.push(token);
                chars = new_chars;
                hit = true;
                break;
            }
        }

        if !hit {
            return Err(format!("Unexpected token: {:?}", chars.iter().next()));
        }
    }

    Ok(res)
}
