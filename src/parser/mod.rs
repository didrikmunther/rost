use crate::lexer::Block;

use self::{definition::AST, error::ParserError, parser::Parser};

pub mod definition;
pub mod util;

mod addition;
mod assignment;
mod comparison;
mod error;
mod function_call;
mod function_declaration;
mod multiplication;
mod parenthesis;
mod parser;
mod primary;
mod return_statement;
mod unexpected;

pub fn parse<'a>(document: &'a Vec<Block>) -> Result<AST, ParserError> {
    Parser::new(document).parse()
}
