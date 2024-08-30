use std::net::{TcpListener, TcpStream};

use connection::HttpConnection;
use response::{HttpResponse, HttpStatus};

mod connection;
mod request;
mod response;

const LOCAL_SOCK: &'static str = "127.0.0.1:8001";

fn handle_client(stream: TcpStream) -> std::io::Result<()> {
    // 1. build a wrapper HttpConn struct that owns the TcpStream
    // 2. conn.get_next_request() -> HttpRequest|Error
    // 3.  ... much logic ...
    // 4. conn.send_reply(reply: HttpResponse) -> Ok|Error
    let mut conn = HttpConnection::new(stream);
    let request = conn.get_next_request()?;
    println!("{:?}", request);
    let mut response = HttpResponse::with_status(HttpStatus::OK);
    response.add_header("Content-Length", "0");
    conn.send_response(response);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(LOCAL_SOCK)?;
    println!("Now listening on {}...", LOCAL_SOCK);
    loop {
        for stream in listener.incoming() {
            // these should be spun off into a thread ig
            handle_client(stream?)?;
        }
    }

    Ok(())
}
