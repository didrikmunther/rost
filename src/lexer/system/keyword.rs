use super::{identifier::IdentifierLexer, Keyword, Letter, Lexer, LexerError, Token};
use phf::phf_map;

static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "let" => Keyword::Let,
    "int" => Keyword::Int,
    "string" => Keyword::String,
    "bool" => Keyword::Bool,
    "fn" => Keyword::Fn,
    "return" => Keyword::Return,
    "if" => Keyword::If,
    "else" => Keyword::Else,
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
    fn lex<'a>(&self, chars: &'a [Letter]) -> Result<Option<(Token, usize)>, LexerError> {
        match self.identifier_lexer.lex(chars) {
            Ok(Some((Token::Identifier(identifier), pos))) => {
                Ok(if let Some(keyword) = KEYWORDS.get(&identifier) {
                    Some((Token::Keyword(*keyword), pos))
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
    use crate::lexer::letter::get_letters;

    #[test]
    fn keyword_works() {
        let letters = &get_letters("let");
        let lexed = KeywordLexer::new().lex(letters);

        assert_eq!(lexed, Ok(Some((Token::Keyword(Keyword::Let), 3))));
    }
}
