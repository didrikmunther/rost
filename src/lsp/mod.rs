use std::collections::VecDeque;
use std::ops::Range;

use lsp_types::{
    DidChangeTextDocumentParams, GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverParams,
    HoverProviderCapability, InitializeParams, InitializeResult, Location, Position as LSPPosition,
    Range as LSPRange, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::fs;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

use crate::compiler::program::ir::{Variable, VariableKind};
use crate::compiler::program::Program;
use crate::error::RostError;
use crate::lexer::{self, Block, Token};
use crate::parser::definition::Ast;
use crate::{compiler, parser};

#[derive(Serialize, Deserialize, Debug)]
struct LspRequest {
    jsonrpc: String,
    id: Option<u64>,
    method: String,
    params: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct LspResponse {
    jsonrpc: String,
    id: Option<u64>,
    result: Option<Value>,
    error: Option<Value>,
}

macro_rules! match_lsp_methods {
    { $( $method:literal => $param_enum:ident ( $param_type:ty ) ),* $(,)? } => {
        #[derive(Debug)]
        enum LspParams {
            $(
                $param_enum($param_type),
            )*
        }

        fn get_params(request: &LspRequest) -> Option<LspParams> {
            match request.method.as_str() {
                $(
                    $method => {
                        Some(LspParams::$param_enum(serde_json::from_value::<$param_type>(
                            request.params.as_ref().unwrap().clone()
                        ).unwrap()))
                    }
                ),*
                _ => None,
            }
        }
    };
}

match_lsp_methods! {
    "initialize" => Initialize(InitializeParams),
    "textDocument/didChange" => DidChangeTextDocument(DidChangeTextDocumentParams),
    "textDocument/hover" => Hover(HoverParams),
    "textDocument/definition" => Definition(GotoDefinitionParams),
}

fn strip_file_protocol(content: &str) -> String {
    content.replace("file://", "")
}

fn get_row_col_vecs(text: &str) -> (VecDeque<u32>, VecDeque<u32>) {
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

fn pos_to_row_col(text: &str, pos: &Range<usize>) -> LSPRange {
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

fn row_col_to_pos(text: &str, pos: &LSPPosition) -> usize {
    let (row_values, column_values) = get_row_col_vecs(text);

    let row = row_values.iter().position(|&x| x >= pos.line).unwrap();
    let col = column_values
        .iter()
        .position(|&x| x >= pos.character)
        .unwrap();

    row + col
}

fn find_block<'a>(
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

fn get_processed_code(text: &str, uri: &str) -> Option<(Vec<Block>, Ast, Program)> {
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

fn get_variable_at_position<'a>(
    text: &str,
    block: &Block,
    pos: LSPPosition,
    program: &'a Program,
) -> Option<&'a Variable> {
    let pos = row_col_to_pos(text, &pos);
    let scope = program
        .scope_lookup
        .get(&pos)
        .and_then(|&v| program.scopes.get(v))?;

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

pub struct LSPServer {
    pub reader: BufReader<OwnedReadHalf>,
    pub writer: OwnedWriteHalf,
}

impl LSPServer {
    async fn read_rpc_message(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut content_length = String::new();
        self.reader.read_line(&mut content_length).await?;

        let content_length: usize = content_length.split(": ").collect::<Vec<&str>>()[1]
            .trim()
            .parse()
            .unwrap();

        let mut empty_line = String::new();
        self.reader.read_line(&mut empty_line).await?;

        let mut buffer = vec![0; content_length];
        self.reader.read_exact(&mut buffer).await?;

        Ok(String::from_utf8(buffer)?)
    }

    async fn write_rpc_message(&mut self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content_length = message.len();
        let header = format!("Content-Length: {}\r\n\r\n", content_length);
        let msg = format!("{}{}", header, message);

        self.writer.write_all(msg.as_bytes()).await?;

        Ok(())
    }

    async fn write_lsp_message(
        &mut self,
        id: Option<u64>,
        deserializable: &impl Serialize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = LspResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(serde_json::to_value(deserializable)?),
            error: None,
        };

        let message = serde_json::to_string(&response)?;
        self.write_rpc_message(message.as_ref()).await
    }

    async fn write_empty_response(
        &mut self,
        id: Option<u64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = LspResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: None,
        };

        let message = serde_json::to_string(&response)?;
        self.write_rpc_message(message.as_ref()).await
    }

    async fn handle_params(
        &mut self,
        request: LspRequest,
        params: LspParams,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match params {
            LspParams::Initialize(_params) => {
                eprintln!("Initializing ...");

                let result = InitializeResult {
                    capabilities: ServerCapabilities {
                        hover_provider: Some(HoverProviderCapability::Simple(true)),
                        text_document_sync: Some(TextDocumentSyncCapability::Kind(
                            TextDocumentSyncKind::FULL,
                        )),
                        definition_provider: Some(lsp_types::OneOf::Left(true)),
                        ..Default::default()
                    },
                    server_info: None,
                };

                self.write_lsp_message(request.id, &result).await?;
            }
            LspParams::DidChangeTextDocument(params) => {
                eprintln!("Did change {:?}", params);
            }
            LspParams::Definition(params) => {
                eprintln!("Defining ...");

                let pos_params = params.text_document_position_params;
                let pos = pos_params.position;
                let uri = pos_params.text_document.uri;
                let text = &fs::read_to_string(strip_file_protocol(uri.as_str())).await?;

                let (lexed, _parsed, program) = match get_processed_code(text, uri.as_str()) {
                    Some(v) => v,
                    None => {
                        self.write_empty_response(request.id).await?;
                        return Ok(());
                    }
                };

                let Some((block, _block_range)) = find_block(&lexed, text, pos) else {
                    self.write_empty_response(request.id).await?;
                    return Ok(());
                };

                let variable = get_variable_at_position(text, block, pos, &program);

                let Some(variable) = variable else {
                    self.write_empty_response(request.id).await?;
                    return Ok(());
                };

                let variable_pos = pos_to_row_col(text, &variable.declaration_pos);

                let result = GotoDefinitionResponse::Scalar(Location {
                    uri,
                    range: variable_pos,
                });

                self.write_lsp_message(request.id, &result).await?;
            }
            LspParams::Hover(params) => {
                let pos_params = params.text_document_position_params;
                let uri = pos_params.text_document.uri;
                let pos = pos_params.position;

                let text = &fs::read_to_string(strip_file_protocol(uri.as_str())).await?;

                let (lexed, _parsed, program) = match get_processed_code(text, uri.as_str()) {
                    Some(v) => v,
                    None => {
                        self.write_empty_response(request.id).await?;
                        return Ok(());
                    }
                };

                let Some((block, block_range)) = find_block(&lexed, text, pos) else {
                    self.write_empty_response(request.id).await?;
                    return Ok(());
                };

                let variable = get_variable_at_position(text, block, pos, &program);

                let Some(variable) = variable else {
                    self.write_empty_response(request.id).await?;
                    return Ok(());
                };

                let kind = match variable.kind {
                    VariableKind::Normal(_) => "Variable",
                    VariableKind::DeclaredFunction(_) => "Function",
                };

                let contents =
                    lsp_types::HoverContents::Scalar(lsp_types::MarkedString::from_markdown(
                        format!(
                            "<{:?}> [{:?}]: {kind:?}",
                            variable.identifier, variable.scope
                        )
                        .to_string(),
                    ));

                let result = Hover {
                    contents,
                    range: Some(block_range),
                };

                self.write_lsp_message(request.id, &result).await?;
            }
        }

        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let content = self.read_rpc_message().await?;

            let request = serde_json::from_str::<LspRequest>(content.as_str()).unwrap();
            let params = get_params(&request);

            eprintln!("Method: {}", request.method);

            match params {
                Some(params) => self.handle_params(request, params).await?,
                None => eprintln!("Unknown method: {}", request.method),
            }
        }
    }
}
