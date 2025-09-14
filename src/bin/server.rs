use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};

fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:50001")?;
    println!("Server listening on port 50001. Press Ctrl+C to stop.");

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> anyhow::Result<()> {
    println!("New connection: {}", stream.peer_addr()?);

    let mut buf = [0; 128];
    stream.read(&mut buf)?;
    println!("Received message: {}", String::from_utf8_lossy(&buf));

    let response = "Echo! You said: ".to_string() + &String::from_utf8_lossy(&buf);
    println!("Sending response: {}", response);
    stream.write_all(response.as_bytes())?;

    println!("Dropping connection: {}", stream.peer_addr()?);
    Ok(())
}
