use file_content::FileContents;
use lifecycle::EmptyClientNotification;
use lsp_types::{
    DidChangeTextDocumentParams, DidOpenTextDocumentParams, GotoDefinitionParams, HoverParams,
    InitializeParams, InitializedParams,
};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use util::{LspNotification, LspRequest, LspResponse};

mod definition;
mod diagnostics;
mod file_content;
mod hover;
mod lifecycle;
mod util;

macro_rules! match_lsp_methods {
    { $( $method:literal => $param_type:ident => $param_handler:ident ),* $(,)? } => {
        async fn handle_request(&mut self, request: &LspRequest) -> Result<(), Box<dyn Error>> {
            match request.method.as_str() {
                $(
                    $method => {
                        let params = serde_json::from_value::<$param_type>(request.params.as_ref().unwrap().clone()).unwrap();
                        let id = request.id.map(|v| format!("Request {}", v.to_string())).unwrap_or("Client notification".to_string());
                        let content = serde_json::to_string_pretty(&request.params.as_ref().unwrap()).unwrap();
                        eprintln!("\n\n{id} ({}): {}", request.method, content);

                        self.$param_handler(request, params).await?;
                    }
                ),*
                _ => {
                    eprintln!("Unknown method: {}", request.method);
                }
            }

            Ok(())
        }
    };
}

pub struct LSPServer {
    pub reader: BufReader<OwnedReadHalf>,
    pub writer: OwnedWriteHalf,
    pub id_method_lookup: HashMap<u64, String>,
    pub file_contents: FileContents,
}

impl LSPServer {
    match_lsp_methods! {
        "initialize" => InitializeParams => handle_initialize,
        "initialized" => InitializedParams => handle_initialized,
        "exit" => EmptyClientNotification => handle_exit,
        "textDocument/definition" => GotoDefinitionParams => handle_goto_definition,
        "textDocument/hover" => HoverParams => handle_hover,
        "textDocument/didChange" => DidChangeTextDocumentParams => handle_did_change,
        "textDocument/didOpen" => DidOpenTextDocumentParams => handle_did_open,
    }

    pub async fn create_and_connect(socket: u16) -> Result<Self, Box<dyn std::error::Error>> {
        eprintln!("[Connecting to the language server at port {socket} ...]");

        let (reader, writer) = TcpStream::connect(format!("127.0.0.1:{}", socket))
            .await?
            .into_split();
        let reader = BufReader::new(reader);

        Ok(Self {
            reader,
            writer,
            id_method_lookup: HashMap::new(),
            file_contents: FileContents::default(),
        })
    }

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

    async fn _write_lsp_message(
        &mut self,
        id: Option<u64>,
        result: Option<Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        eprintln!(
            "\n\nResponse {} ({}): {}",
            id.map(|v| v.to_string()).unwrap_or("?".to_string()),
            id.map(|v| self.id_method_lookup.get(&v).unwrap())
                .unwrap_or(&"unknown method".to_string()),
            result
                .as_ref()
                .map(|value| serde_json::to_string_pretty(&value).unwrap())
                .unwrap_or("".to_string())
        );

        let response = LspResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result,
            error: None,
        };

        let message = serde_json::to_string(&response)?;
        self.write_rpc_message(message.as_ref()).await
    }

    async fn _write_lsp_notification(
        &mut self,
        method: &str,
        params: Option<Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        eprintln!(
            "\n\nServer notification: {}",
            params
                .as_ref()
                .map(|value| serde_json::to_string_pretty(&value).unwrap())
                .unwrap_or("".to_string())
        );

        let response = LspNotification {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
        };

        let message = serde_json::to_string(&response)?;
        self.write_rpc_message(message.as_ref()).await
    }

    async fn write_lsp_message(
        &mut self,
        id: Option<u64>,
        deserializable: &impl Serialize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self._write_lsp_message(id, Some(serde_json::to_value(deserializable)?))
            .await
    }

    async fn write_lsp_notification(
        &mut self,
        method: &str,
        deserializable: &impl Serialize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self._write_lsp_notification(method, Some(serde_json::to_value(deserializable)?))
            .await
    }

    async fn write_empty_response(
        &mut self,
        id: Option<u64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self._write_lsp_message(id, None).await
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let content = self.read_rpc_message().await?;
            let request = serde_json::from_str::<LspRequest>(content.as_str()).unwrap();

            if let Some(id) = request.id {
                self.id_method_lookup.insert(id, request.method.clone());
            }

            self.handle_request(&request).await?;
        }
    }
}
