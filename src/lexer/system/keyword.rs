use super::{identifier::IdentifierLexer, Keyword, Letter, Lexer, Token};
use phf::phf_map;

static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "let" => Keyword::Let,
};

pub struct KeywordLexer {
    identifier_lexer: IdentifierLexer,
}

impl KeywordLexer {
    pub fn new() -> Self {
        Self {
            identifier_lexer: IdentifierLexer::new(),
        }
    }
}

impl Lexer for KeywordLexer {
    fn lex<'a>(&self, chars: &'a [Letter]) -> Option<(Token, &'a [Letter])> {
        match self.identifier_lexer.lex(chars) {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::letter::{get_letters, EOF};

    #[test]
    fn keyword_works() {
        let letters = &get_letters("let");
        let lexed = KeywordLexer::new().lex(letters);
        let rest: &[Letter] = &[EOF];

        assert_eq!(lexed, Some((Token::Keyword(Keyword::Let), rest)));
    }
}