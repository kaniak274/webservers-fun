use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

const PORT: &str = "8888";
const HOST: &str = "127.0.0.1";

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let reader = BufReader::new(&mut stream);

    let data: Vec<String> = reader
        .lines()
        .filter_map(|result| result.ok())
        .collect();

    println!("Received data from client {}: {:#?}", stream.peer_addr().unwrap(), data.join(","));

    Ok(())
}

fn main() -> std::io::Result<()> {
    let socket = TcpListener::bind(format!("{HOST}:{PORT}"))?;

    for stream in socket.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
