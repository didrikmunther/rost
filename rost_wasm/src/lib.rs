use std::{collections::VecDeque, ops::Range};

use rost::{
    backend::{wasm::WasmBackend, Backend},
    error::{RostError, RostErrorElement},
    util::{get_processed_code, CompilerResult},
};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

pub fn get_row_col_vecs(text: &str) -> (VecDeque<u32>, VecDeque<u32>) {
    let new_lines = text
        .chars()
        .map(|c| c == '\n')
        .chain(std::iter::once(true))
        .collect::<Vec<_>>();

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

pub struct LSPPosition {
    pub line: u32,
    pub character: u32,
}

pub struct LSPRange {
    pub start: LSPPosition,
    pub end: LSPPosition,
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

fn create_error_message(code: &str, err: &RostError, element: &RostErrorElement) -> Vec<String> {
    let pos = pos_to_row_col(code, &element.pos);

    vec![
        format!("{}", err.kind),
        format!(
            "[rows {}-{}] {}",
            pos.start.line + 1,
            pos.end.line + 1,
            element.message
        ),
    ]
}

#[wasm_bindgen]
struct WasmResult {
    lexed: Result<String, String>,
    parsed: Option<Result<String, String>>,
    compiled: Option<Result<String, String>>,
    wasm: Option<Result<String, String>>,
}

#[wasm_bindgen]
impl WasmResult {
    pub fn get_lexed(&self) -> Result<String, String> {
        self.lexed.clone()
    }

    pub fn get_parsed(&self) -> Result<String, String> {
        self.parsed.clone().unwrap_or(Ok("".to_string()))
    }

    pub fn get_compiled(&self) -> Result<String, String> {
        self.compiled.clone().unwrap_or(Ok("".to_string()))
    }

    pub fn get_wasm(&self) -> Result<String, String> {
        self.wasm.clone().unwrap_or(Ok("".to_string()))
    }
}

#[wasm_bindgen]
pub fn compile(code: &str) -> WasmResult {
    let return_error = |mut errs: Vec<RostError>| {
        errs.iter_mut()
            .flat_map(|err| {
                err.with_code(Some("rost".to_string()));
                err.to_string();
                err.elements
                    .iter()
                    .flat_map(|element| create_error_message(code, err, element))
            })
            .collect::<Vec<String>>()
            .join("\n")
    };

    let (lexed, parsed, compiled) = match get_processed_code(code, "main.ro") {
        CompilerResult::Lexed(lexed) => {
            return WasmResult {
                lexed: lexed
                    .map(|lexed| format!("{lexed:#?}"))
                    .map_err(|err| return_error(vec![err])),
                parsed: None,
                compiled: None,
                wasm: None,
            }
        }
        CompilerResult::Parsed { lexed, parsed } => {
            return WasmResult {
                lexed: Ok(format!("{lexed:#?}")),
                parsed: Some(
                    parsed
                        .map(|parsed| format!("{parsed:#?}"))
                        .map_err(|err| return_error(vec![err])),
                ),
                compiled: None,
                wasm: None,
            }
        }
        CompilerResult::Compiled {
            lexed,
            parsed,
            compiled,
        } => (lexed, parsed, compiled),
    };

    if !compiled.errors.is_empty() {
        return WasmResult {
            lexed: Ok(format!("{lexed:#?}")),
            parsed: Some(Ok(format!("{parsed:#?}"))),
            compiled: Some(Err(return_error(compiled.errors))),
            wasm: None,
        };
    }

    let wasm = match WasmBackend.generate(&compiled.program) {
        Ok(generated) => Ok(generated),
        Err(err) => Err(return_error(vec![err])),
    };

    WasmResult {
        lexed: Ok(format!("{lexed:#?}")),
        parsed: Some(Ok(format!("{parsed:#?}"))),
        compiled: Some(Ok(format!("{:#?}", compiled.program))),
        wasm: Some(wasm),
    }
}
