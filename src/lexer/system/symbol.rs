use super::{Keyword, Letter, Lexer, LexerError, Token};
use phf::phf_map;

// Symbols are not alphanumerical
static SYMBOLS: phf::Map<&'static str, Keyword> = phf_map! {
    "=" => Keyword::Equals,
    ";" => Keyword::Semicolon,
    "=>" => Keyword::Arrow,
    "+" => Keyword::Plus,
    "-" => Keyword::Minus,
    "*" => Keyword::Asterix,
    "/" => Keyword::Slash,
    "(" => Keyword::ParLeft,
    ")" => Keyword::ParRight,
    "{" => Keyword::BracketLeft,
    "}" => Keyword::BracketRight,
    "," => Keyword::Comma,
    ":" => Keyword::Colon
};

pub struct SymbolLexer;

impl SymbolLexer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Lexer for SymbolLexer {
    fn lex<'a>(&self, chars: &'a [Letter]) -> Result<Option<(Token, usize)>, LexerError> {
        let mut buf = Vec::<char>::new();

        for (mut i, &(_pos, cur, eof)) in chars.into_iter().enumerate() {
            if buf.is_empty() && cur.is_whitespace() {
                continue;
            }

            let mut word: String = buf.iter().collect();

            let potential_symbols = SYMBOLS
                .keys()
                .filter(|k| word.len() <= k.len() && k.starts_with(&word));

            // Backtrack through word to find a symbol
            if eof || potential_symbols.count() <= 0 {
                i += 1;
                word += " "; // Add padding for a do-while loop-style thing here
                while let Some(_) = word.pop() {
                    i -= 1;
                    if let Some(&symbol) = SYMBOLS.get(&word) {
                        return Ok(Some((Token::Keyword(symbol), i)));
                    }
                }

                return Ok(None);
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
    fn symbol_works() {
        let letters = &get_letters("=>");
        let lexed = SymbolLexer::new().lex(letters);

        assert_eq!(lexed, Ok(Some((Token::Keyword(Keyword::Arrow), 2))));
    }
}
