use super::{identifier::IdentifierLexer, Keyword, Letter, Lexer, LexerError, Token};
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
    fn lex<'a>(&self, chars: &'a [Letter]) -> Result<Option<(Token, &'a [Letter])>, LexerError> {
        match self.identifier_lexer.lex(chars) {
            Ok(Some((Token::Identifier(identifier), new_chars))) => {
                Ok(if let Some(keyword) = KEYWORDS.get(&identifier) {
                    Some((Token::Keyword(*keyword), new_chars))
                } else {
                    None
                })
            }
            _ => Ok(None),
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

        assert_eq!(lexed, Ok(Some((Token::Keyword(Keyword::Let), rest))));
    }
}
