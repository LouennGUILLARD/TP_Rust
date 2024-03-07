
use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}};

use clap::Parser;

//const  PONG: &str = "PONG\n";
const  BUFFER_SIZE: usize = 2048;

#[derive(Parser)]
struct Options {
    host : String,
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let options = Options::parse();
    let serveur = TcpListener::bind(format!("{}:{}",options.host,options.port)).await?;
    loop {
        let (mut stream, _) = serveur.accept().await?;
        tokio::spawn(async move {
            let (mut reader, mut writer) = stream.split();
            let mut buf = [0;BUFFER_SIZE];
            let bytes_reader=reader.read(&mut buf).await?;
            let received_message = std::str::from_utf8(&buf[..bytes_reader])?;
            writer.write_all(received_message.as_bytes()).await?;
            writer.flush().await?;
            Ok::<(), anyhow::Error>(())
        });
    }
}