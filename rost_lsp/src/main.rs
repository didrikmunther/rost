use clap::Parser;
use lsp::LSPServer;

mod lsp;

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

    LSPServer::create_and_connect(args.socket)
        .await?
        .run()
        .await
}
