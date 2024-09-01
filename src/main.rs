use std::{
    io::{Error, ErrorKind},
    net::{TcpListener, TcpStream},
};

use connection::HttpConnection;
use request::HttpRequest;
use response::{HttpResponse, HttpStatus};

mod connection;
mod request;
mod response;

const LOCAL_SOCK: &'static str = "127.0.0.1:8001";

fn handle_client(stream: TcpStream) -> std::io::Result<()> {
    let mut conn = HttpConnection::new(stream);
    let request = conn.get_next_request()?;
    let response = handle_request(request)?;
    conn.send_response(response);
    Ok(())
}

fn handle_request(request: HttpRequest) -> Result<HttpResponse, std::io::Error> {
    let mut response = HttpResponse::with_status(HttpStatus::OK);
    response.add_header("Content-Length", "0");

    validate_request(&request);

    // are we trying to get a file?
    // are we allowed to get the file?
    // calculate content-length and put the file into response body

    Ok(response)
}

fn validate_request(request: &HttpRequest) -> Result<(), std::io::Error> {
    match request.read_header("Host") {
        Some(_) => Ok(()),
        None => Err(Error::new(
            ErrorKind::InvalidInput,
            "Missing required Host header",
        )),
    }?;

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
