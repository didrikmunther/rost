pub use super::{Keyword, Letter, LexerError, LexerErrorKind, Literal, Token};

pub mod comment;
pub mod identifier;
pub mod keyword;
pub mod literal_bool;
pub mod literal_number;
pub mod string;
pub mod symbol;

pub use comment::CommentLexer;
pub use identifier::IdentifierLexer;
pub use keyword::KeywordLexer;
pub use literal_bool::LiteralBoolLexer;
pub use literal_number::LiteralNumberLexer;
pub use string::StringLexer;
pub use symbol::SymbolLexer;

pub trait Lexer {
    fn lex(&self, chars: &[Letter]) -> Result<Option<(Token, usize)>, LexerError>;
}
