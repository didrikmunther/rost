use std::collections::VecDeque;
use std::ops::Range;

use lsp_types::{Position as LSPPosition, Range as LSPRange};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::compiler::program::ir::Variable;
use crate::compiler::program::Program;
use crate::error::RostError;
use crate::lexer::{self, Block, Token};
use crate::parser::definition::Ast;
use crate::{compiler, parser};

#[derive(Serialize, Deserialize, Debug)]
pub struct LspRequest {
    jsonrpc: String,
    pub id: Option<u64>,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LspResponse {
    pub jsonrpc: String,
    pub id: Option<u64>,
    pub result: Option<Value>,
    pub error: Option<Value>,
}

pub fn strip_file_protocol(content: &str) -> String {
    content.replace("file://", "")
}

pub fn get_row_col_vecs(text: &str) -> (VecDeque<u32>, VecDeque<u32>) {
    let new_lines = text
        .chars()
        .map(|c| c == '\n')
        .chain(std::iter::once(true))
        .chain(std::iter::once(true));

    let mut row_values = new_lines
        .clone()
        .scan(0, |acc, x| {
            *acc += if x { 1 } else { 0 };
            Some(*acc)
        })
        .collect::<VecDeque<u32>>();

    row_values.rotate_right(1);
    row_values[0] = 0;

    let column_values = new_lines
        .scan(0, |acc, x| {
            if !x {
                *acc += 1;
                Some(*acc - 1)
            } else {
                *acc = 0;
                Some(*acc)
            }
        })
        .collect::<VecDeque<u32>>();

    (row_values, column_values)
}

pub fn pos_to_row_col(text: &str, pos: &Range<usize>) -> LSPRange {
    let (row_values, column_values) = get_row_col_vecs(text);

    LSPRange {
        start: LSPPosition {
            character: *column_values.get(pos.start).unwrap(),
            line: *row_values.get(pos.start).unwrap(),
        },
        end: LSPPosition {
            character: *column_values.get(pos.end).unwrap(),
            line: *row_values.get(pos.end).unwrap(),
        },
    }
}

pub fn row_col_to_pos(text: &str, pos: &LSPPosition) -> usize {
    let (row_values, column_values) = get_row_col_vecs(text);

    let row = row_values.iter().position(|&x| x >= pos.line).unwrap();
    let col = column_values
        .iter()
        .position(|&x| x >= pos.character)
        .unwrap();

    row + col
}

pub fn find_block<'a>(
    lexed: &'a [Block],
    text: &str,
    pos: LSPPosition,
) -> Option<(&'a Block, LSPRange)> {
    let (row_values, column_values) = get_row_col_vecs(text);

    let block = lexed.iter().find(|block| {
        *column_values.get(block.pos.start).unwrap() <= pos.character
            && *column_values.get(block.pos.end).unwrap() >= pos.character
            && *row_values.get(block.pos.start).unwrap() <= pos.line
            && *row_values.get(block.pos.end).unwrap() >= pos.line
    })?;

    let block_range = LSPRange {
        start: LSPPosition {
            character: *column_values.get(block.pos.start).unwrap(),
            line: *row_values.get(block.pos.start).unwrap(),
        },
        end: LSPPosition {
            character: *column_values.get(block.pos.end).unwrap(),
            line: *row_values.get(block.pos.end).unwrap(),
        },
    };

    Some((block, block_range))
}

pub fn get_processed_code(text: &str, uri: &str) -> Option<(Vec<Block>, Ast, Program)> {
    let print_error = |mut err: RostError, text: &str, file: &str| {
        eprintln!(
            "{}",
            err.with_code(Some(text.to_string()))
                .with_file(Some(file.to_string()))
        );
    };

    let lexed = match lexer::lex(text) {
        Ok(v) => v,
        Err(err) => {
            print_error(err.into(), text, uri);
            return None;
        }
    };

    let parsed = match parser::parse(&lexed) {
        Ok(v) => v,
        Err(err) => {
            print_error(err.into(), text, uri);
            return None;
        }
    };

    let compiled = match compiler::compile(parsed.clone()) {
        Ok(v) => v,
        Err(err) => {
            print_error(err.into(), text, uri);
            return None;
        }
    };

    Some((lexed, parsed, compiled))
}

pub fn get_variable_at_position<'a>(
    text: &str,
    block: &Block,
    pos: LSPPosition,
    program: &'a Program,
) -> Option<&'a Variable> {
    let pos = row_col_to_pos(text, &pos);
    let scope = program
        .scope_lookup
        .find(pos, pos)
        .next()
        .and_then(|v| program.scopes.get(v.val))?;

    let variable = match &block.token {
        Token::Identifier(identifier) => scope
            .variable_lookup
            .get(identifier)
            .and_then(|&v| program.variables.get(v)),
        _ => {
            return None;
        }
    };

    variable
}
