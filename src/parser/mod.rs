use crate::lexer::Block;

use self::{definition::AST, error::ParserError, parser::Parser};

pub mod definition;
pub mod types;
pub mod util;

mod addition;
mod assignment;
mod comparison;
mod error;
mod function_call;
mod function_declaration;
mod if_statement;
mod multiplication;
mod parenthesis;
mod parser;
mod primary;
mod reference;
mod return_statement;
mod unexpected;
mod while_statement;

pub fn parse<'a>(document: &'a Vec<Block>) -> Result<AST, ParserError> {
    Parser::new(document).parse()
}
