use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
};

mod http;

const LOCAL_SOCK: &'static str = "127.0.0.1:8001";

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    // 1. build a wrapper HttpConn struct that owns the TcpStream
    // 2. conn.get_next_request() -> HttpRequest|Error
    // 3.  ... much logic ...
    // 4. conn.send_reply(reply: HttpResponse) -> Ok|Error
    let mut read_buf = String::new();
    let stream_reader = BufReader::new(&stream);
    let mut request: http::HttpRequest;
    for line in stream_reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
        if line.is_empty() {
            break;
        }
    }
    let reply = http::HttpResponse { response_code: 200 };
    stream.write(b"HTTP/1.1 404\r\nContent-Length: 0\r\n")?;
    stream.flush();
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(LOCAL_SOCK)?;
    println!("Now listening on {}...", LOCAL_SOCK);
    for stream in listener.incoming() {
        // these should be spun off into a thread ig
        handle_client(stream?)?;
    }

    Ok(())
}
