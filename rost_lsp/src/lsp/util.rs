use std::collections::VecDeque;
use std::ops::Range;

use lsp_types::{Position as LSPPosition, Range as LSPRange};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use rost::compiler::program::ir::Variable;
use rost::compiler::program::Program;
use rost::error::RostError;
use rost::lexer::{self, Block, Token};
use rost::parser::definition::Ast;
use rost::{compiler, parser};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct LspNotification {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>,
}

pub fn strip_file_protocol(content: &str) -> String {
    content.replace("file://", "")
}

pub fn get_row_col_vecs(text: &str) -> (VecDeque<u32>, VecDeque<u32>) {
    let new_lines = text
        .chars()
        .map(|c| c == '\n')
        .chain(std::iter::once(true))
        .collect::<Vec<_>>();

    // eprintln!(
    //     "new_lines :{:?}",
    //     new_lines
    //         .iter()
    //         .map(|&v| if v { 1 } else { 0 })
    //         .collect::<Vec<_>>()
    // );

    let mut row_values = new_lines
        .iter()
        .scan(0, |acc, &x| {
            *acc += if x { 1 } else { 0 };
            Some(*acc)
        })
        .collect::<VecDeque<u32>>();

    row_values.push_back(0);
    row_values.rotate_right(1);

    let mut column_values = new_lines
        .iter()
        .scan(0, |acc, x| {
            if !x {
                *acc += 1;
                Some(*acc - 1)
            } else {
                let tmp = *acc;
                *acc = 0;
                Some(tmp)
            }
        })
        .collect::<VecDeque<u32>>();

    column_values.push_back(0);

    // eprintln!("row_values:{:?}", row_values);
    // eprintln!("col_values:{:?}", column_values);

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

pub struct Compiled {
    pub program: Program,
    pub errors: Vec<RostError>,
}

#[allow(dead_code)]
pub enum CompilerResult {
    Lexed(Result<Vec<Block>, RostError>),
    Parsed {
        lexed: Vec<Block>,
        parsed: Result<Ast, RostError>,
    },
    Compiled {
        lexed: Vec<Block>,
        parsed: Ast,
        compiled: Compiled,
    },
}

pub fn get_processed_code(text: &str, uri: &str) -> CompilerResult {
    let get_error = |mut err: RostError| -> RostError {
        err.with_code(Some(text.to_string()))
            .with_file(Some(uri.to_string()));

        err
    };

    let lexed = match lexer::lex(text) {
        Ok(v) => v,
        Err(err) => return CompilerResult::Lexed(Err(get_error(err.into()))),
    };

    let parsed = match parser::parse(&lexed) {
        Ok(v) => v,
        Err(err) => {
            return CompilerResult::Parsed {
                lexed,
                parsed: Err(get_error(err.into())),
            }
        }
    };

    let program = compiler::compile(parsed.clone());
    let errors = program
        .errors
        .clone()
        .into_iter()
        .map(|e| get_error(e.into()))
        .collect();

    CompilerResult::Compiled {
        lexed,
        parsed,
        compiled: Compiled { program, errors },
    }
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
            .and_then(|v| program.variables.get(v.get_id())),
        _ => {
            return None;
        }
    };

    variable
}

pub fn create_code_block(lang: &str, code: &str) -> String {
    format!("```{}\n{}\n```", lang, code)
}
