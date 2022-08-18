use crate::lexer::Block;

use self::{definition::AST, error::ParserError, parser::Parser};

mod addition;
mod assignment;
pub mod definition;
mod error;
mod function_call;
mod parser;
mod primary;
mod unexpected;
mod util;

pub fn parse<'a>(document: &'a Vec<Block>) -> Result<AST, ParserError> {
    Parser::new(document).parse()
}
