use clap::Parser;
use std::net::UdpSocket;
use anyhow::Ok;

const  PING: &str = "PING";

#[derive(Parser)]
struct Options {
    host : String,
    port: u16,
}

fn main() -> anyhow::Result<()> {
    
    let options = Options::parse();
    let serveur = format!("{}:{}",options.host,options.port);

    let my_socket = UdpSocket::bind("127.0.0.1:0")?;
    my_socket.send_to(PING.as_bytes(),serveur)?;

    Ok(())

}
