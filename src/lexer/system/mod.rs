pub use super::{Keyword, Letter, Literal, Token};

pub mod identifier;
pub mod keyword;
pub mod literal_number;
pub mod symbol;

pub use identifier::IdentifierLexer;
pub use keyword::KeywordLexer;
pub use literal_number::LiteralNumberLexer;
pub use symbol::SymbolLexer;

pub trait Lexer {
    fn lex<'a>(&self, chars: &'a [Letter]) -> Option<(Token, &'a [Letter])>;
}
