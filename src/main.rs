use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
};

use http::HttpConnection;

mod http;

const LOCAL_SOCK: &'static str = "127.0.0.1:8001";

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    // 1. build a wrapper HttpConn struct that owns the TcpStream
    // 2. conn.get_next_request() -> HttpRequest|Error
    // 3.  ... much logic ...
    // 4. conn.send_reply(reply: HttpResponse) -> Ok|Error
    let conn = HttpConnection::new(stream);
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
