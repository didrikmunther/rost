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
mod member;
mod multiplication;
mod parenthesis;
mod parser;
mod primary;
mod reference;
mod return_statement;
mod struct_construction;
mod struct_declaration;
mod unexpected;
mod while_statement;

pub fn parse(document: &Vec<Block>) -> Result<AST, ParserError> {
    Parser::new(document).parse()
}
