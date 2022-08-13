use super::{Keyword, Letter, Lexer, Token};
use phf::phf_map;

// Symbols are not alphanumerical
static SYMBOLS: phf::Map<&'static str, Keyword> = phf_map! {
    "=" => Keyword::Equals,
    ";" => Keyword::Semicolon,
    "=>" => Keyword::Arrow,
    "+" => Keyword::Plus,
    "-" => Keyword::Minus,
};

pub struct SymbolLexer;

impl SymbolLexer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Lexer for SymbolLexer {
    fn lex<'a>(&self, chars: &'a [Letter]) -> Option<(Token, &'a [Letter])> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::letter::{get_letters, EOF};

    #[test]
    fn symbol_works() {
        let letters = &get_letters("=>");
        let lexed = SymbolLexer::new().lex(letters);
        let rest: &[Letter] = &[EOF];

        assert_eq!(lexed, Some((Token::Keyword(Keyword::Arrow), rest)));
    }
}