use std::{io::{Read, Write}, net::TcpStream};

fn main() -> anyhow::Result<()> {
    // Connect to server
    let mut stream = TcpStream::connect("127.0.0.1:50001")?;

    // Send a message to the server
    stream.write_all(b"Hello, World!")?;

    // Read the server's response
    let mut response = [0; 128];
    stream.read(&mut response)?;

    println!("Received from server: {}", String::from_utf8_lossy(&response));

    Ok(())
}