use std::{
    fs::File,
    io::{Error, ErrorKind, Read},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
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
    let response = generate_response(request)?;
    conn.send_response(response)?;
    Ok(())
}

fn generate_response(request: HttpRequest) -> Result<HttpResponse, std::io::Error> {
    let mut response = HttpResponse::with_status(HttpStatus::OK);
    response.add_header("Content-Length", "0");

    validate_request(&request)?;

    // are we trying to get a file?
    // are we allowed to get the file?
    let mut final_path = PathBuf::new();
    let webroot = std::env::current_dir()?;
    println!("pwd: {}", webroot.display());
    final_path.push(&webroot);
    let mut target_path = Path::new(&request.target);
    if target_path.is_absolute() {
        target_path = target_path.strip_prefix("/").unwrap();
    }
    final_path.push(target_path);

    // add a lot of error handling here
    println!("trying to read: {}", final_path.display());
    let file_contents = std::fs::read(final_path)?;
    let body_size = file_contents.len();
    response.add_header("Content-Length", body_size.to_string().as_str());
    response.put_body(file_contents);

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
    let cur_path = std::env::current_dir()?;
    println!("Working directory: {}", cur_path.display());
    loop {
        for stream in listener.incoming() {
            // these should be spun off into a thread ig
            handle_client(stream?)?;
        }
    }

    Ok(())
}
