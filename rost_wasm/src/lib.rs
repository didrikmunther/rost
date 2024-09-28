use rost::{
    backend::{wasm::WasmBackend, Backend},
    compiler,
    error::RostError,
    lexer, parser,
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

#[wasm_bindgen]
pub fn compile(code: &str) -> Result<String, String> {
    let return_error = |mut errs: Vec<RostError>| {
        errs.iter_mut()
            .map(|err| {
                err.with_code(Some("rost".to_string()));
                err.to_string()
            })
            .collect::<Vec<String>>()
            .join("\n")
    };

    let document = match lexer::lex(code) {
        Ok(lexed) => lexed,
        Err(err) => {
            return Err(return_error(vec![err.into()]));
        }
    };

    let parsed = match parser::parse(&document) {
        Ok(program) => program,
        Err(err) => {
            return Err(return_error(vec![err.into()]));
        }
    };

    let program = compiler::compile(parsed);

    if !program.errors.is_empty() {
        return Err(return_error(
            program.errors.into_iter().map(|e| e.into()).collect(),
        ));
    }

    match WasmBackend.generate(&program) {
        Ok(generated) => Ok(generated),
        Err(err) => Err(return_error(vec![err])),
    }
}
