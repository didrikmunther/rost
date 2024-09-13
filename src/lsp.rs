use clap::Parser;
use lsp_types::{
    Hover, HoverParams, HoverProviderCapability, InitializeParams, InitializeResult,
    ServerCapabilities,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::fs;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;

mod compiler;
mod error;
mod lexer;
mod nasm;
mod parser;

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

#[derive(Debug)]
enum LspParams {
    Initialize(InitializeParams),
    Hover(HoverParams),
}

macro_rules! match_lsp_methods {
    ( $request:expr, { $( $method:literal => $param_enum:ident => $param_type:ty ),* $(,)? } ) => {
        match $request.method.as_str() {
            $(
                $method => {
                    Some(LspParams::$param_enum(serde_json::from_value::<$param_type>(
                        $request.params.as_ref().unwrap().clone()
                    ).unwrap()))
                }
            ),*
            _ => None,
        }
    };
}

async fn read_rpc_message(
    reader: &mut BufReader<OwnedReadHalf>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut content_length = String::new();
    reader.read_line(&mut content_length).await?;

    let content_length: usize = content_length.split(": ").collect::<Vec<&str>>()[1]
        .trim()
        .parse()
        .unwrap();

    let mut empty_line = String::new();
    reader.read_line(&mut empty_line).await?;

    let mut buffer = vec![0; content_length];
    reader.read_exact(&mut buffer).await?;

    Ok(String::from_utf8(buffer)?)
}

async fn write_rpc_message(
    stream: &mut OwnedWriteHalf,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let content_length = message.len();
    let header = format!("Content-Length: {}\r\n\r\n", content_length);
    let msg = format!("{}{}", header, message);

    stream.write_all(msg.as_bytes()).await?;

    Ok(())
}

async fn write_lsp_message(
    stream: &mut OwnedWriteHalf,
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
    write_rpc_message(stream, message.as_ref()).await
}

fn get_params(request: &LspRequest) -> Option<LspParams> {
    match_lsp_methods!(request, {
        "initialize" => Initialize => InitializeParams,
        "textDocument/hover" => Hover => HoverParams,
    })
}

fn strip_file_protocol(content: &str) -> String {
    content.replace("file://", "")
}

async fn handle_params(
    request: LspRequest,
    params: LspParams,
    writer: &mut OwnedWriteHalf,
) -> Result<(), Box<dyn std::error::Error>> {
    match params {
        LspParams::Initialize(_params) => {
            eprintln!("Initializing ...");

            let result = InitializeResult {
                capabilities: ServerCapabilities {
                    hover_provider: Some(HoverProviderCapability::Simple(true)),
                    ..Default::default()
                },
                server_info: None,
            };

            write_lsp_message(writer, request.id, &result).await?;
        }
        LspParams::Hover(params) => {
            eprintln!("Hovering ...");
            eprintln!("{:?}", serde_json::to_string(&params)?);

            let pos_params = params.text_document_position_params;
            let uri = strip_file_protocol(pos_params.text_document.uri.as_str());
            // let pos = pos_params.position;

            let text = &fs::read_to_string(uri).await?;

            let document = match lexer::lex(text) {
                Ok(lexed) => Some(lexed),
                Err(_err) => {
                    // print_error(err.into());
                    None
                }
            };

            let text = document
                .and_then(|document| document.into_iter().find(|v| v.pos.start > 0))
                .map(|block| format!("{:?}", block.kind))
                .unwrap_or("".to_string());

            let contents =
                lsp_types::HoverContents::Scalar(lsp_types::MarkedString::from_markdown(text));

            let result = Hover {
                contents,
                range: None,
            };

            write_lsp_message(writer, request.id, &result).await?;
        }
    }

    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The socket port to connect to the language server
    #[arg(short, long, default_value_t = 8080)]
    socket: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    eprintln!("{:?}", args);
    eprintln!("Connecting to the language server ...");

    let (reader, mut writer) = TcpStream::connect(format!("127.0.0.1:{}", args.socket))
        .await?
        .into_split();
    let mut reader = BufReader::new(reader);

    loop {
        let content = read_rpc_message(&mut reader).await?;
        let request = serde_json::from_str::<LspRequest>(content.as_str()).unwrap();
        let params = get_params(&request);

        match params {
            Some(params) => handle_params(request, params, &mut writer).await?,
            None => eprintln!("Unknown method: {}", request.method),
        }
    }
}
