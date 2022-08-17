use crate::lexer::Block;

use self::{definition::AST, error::ParserError, parser::Parser};

mod addition;
pub mod definition;
mod error;
mod function_call;
mod parser;
mod primary;

pub fn parse<'a>(document: &'a Vec<Block>) -> Result<AST, ParserError> {
    Parser::new(document).parse()
}
