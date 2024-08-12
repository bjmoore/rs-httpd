use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

mod http;

fn handle_client(stream: TcpStream) -> std::io::Result<()> {
    // what does the serial req-reply workflow look like?
    // when do we close out?
    // we keep conn open and send replies until client closes the connection, OR
    // until one of various H1.1/1,0 closure conditions
    let mut read_buf = String::new();
    let mut stream_reader = BufReader::new(stream);
    let mut request: http::HttpRequest;
    loop {
        read_buf.clear();
        stream_reader.read_line(&mut read_buf)?;
        print!("{}", read_buf);
        if read_buf == "\r\n" {
            break;
        }
    }
    let reply = http::HttpResponse { response_code: 200 };
    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:8001")?;
    for stream in listener.incoming() {
        // these should be spun off into a thread ig
        handle_client(stream?)?;
    }

    Ok(())
}
