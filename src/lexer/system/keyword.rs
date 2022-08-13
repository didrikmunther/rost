use super::{identifier::identifier_lexer, Keyword, Letter, Token};
use phf::phf_map;

static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "let" => Keyword::Let,
};

pub fn keyword_lexer<'a>(chars: &'a [Letter]) -> Option<(Token, &'a [Letter])> {
    match identifier_lexer(chars) {
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
