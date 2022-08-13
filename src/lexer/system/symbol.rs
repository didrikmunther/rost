use super::{Keyword, Letter, Token};
use phf::phf_map;

// Symbols are not alphanumerical
static SYMBOLS: phf::Map<&'static str, Keyword> = phf_map! {
    "=" => Keyword::Equals,
    ";" => Keyword::Semicolon,
    "=>" => Keyword::Equals // todo: temp
};

pub fn symbol_lexer<'a>(chars: &'a [Letter]) -> Option<(Token, &'a [Letter])> {
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
