use std::{
    io::{BufRead, BufReader, Read},
    net::{TcpListener, TcpStream},
};

fn handle_client(stream: TcpStream) -> std::io::Result<()> {
    let mut read_buf = String::new();
    let mut stream_reader = BufReader::new(stream);
    stream_reader.read_line(&mut read_buf)?;
    println!("{}", read_buf);
    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:8001")?;
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}
