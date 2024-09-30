use crate::{
    compiler::{self, program::Program},
    error::RostError,
    lexer::{self, Block},
    parser::{self, definition::Ast},
};

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
