use clap::Parser;
use exquisite_corpse::make_reponse;
use std::net::UdpSocket;

#[derive(Parser)]
struct Options {
    host : String,
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let options = Options::parse();
    let serveur = UdpSocket::bind(format!("{}:{}",options.host,options.port))?;
    let mut text = String::from(""); // Change the type of `text` to `&mut String`
    let mut buf = vec![0u8; 8888];

    loop {
        let (size, addr) = serveur.recv_from(&mut buf)?;
        
        let received_message = std::str::from_utf8(&buf[..size])?;
        println!("Received '{}' from {}", received_message, addr); 
        serveur.send_to(make_reponse(received_message, &mut text).as_bytes(), &addr)?;
    }
}
