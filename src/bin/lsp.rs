use clap::Parser;
use rost::lsp::LSPServer;
use tokio::{io::BufReader, net::TcpStream};

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

    let (reader, writer) = TcpStream::connect(format!("127.0.0.1:{}", args.socket))
        .await?
        .into_split();
    let reader = BufReader::new(reader);

    let mut server = LSPServer { reader, writer };

    server.run().await?;

    Ok(())
}
