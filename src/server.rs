use std::io::{Error, ErrorKind};
use std::path::Path;
use std::{
    net::{TcpListener, TcpStream},
    path::PathBuf,
};

use crate::response::HttpStatus;
use crate::{connection::HttpConnection, request::HttpRequest, response::HttpResponse};

pub struct HttpServer {
    listener: TcpListener,
}

impl HttpServer {
    pub fn new(addr: &str) -> std::io::Result<Self> {
        Ok(Self {
            listener: TcpListener::bind(addr)?,
        })
    }

    pub fn run(&self) -> std::io::Result<()> {
        loop {
            for stream in self.listener.incoming() {
                // these should be spun off into a thread ig
                handle_client(stream?)?;
            }
        }
    }
}

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
    let path = route_request_to_local_fs(&request)?;

    // add a lot of error handling here
    println!("trying to read: {}", path.display());
    let file_contents = std::fs::read(path)?;
    let body_size = file_contents.len();
    response.add_header("Content-Length", body_size.to_string().as_str());
    response.put_body(file_contents);

    Ok(response)
}

fn route_request_to_local_fs(request: &HttpRequest) -> std::io::Result<PathBuf> {
    let mut final_path = PathBuf::new();
    let webroot = std::env::current_dir()?;
    println!("pwd: {}", webroot.display());
    final_path.push(&webroot);
    let mut target_path = Path::new(&request.target);
    if target_path.is_absolute() {
        target_path = target_path.strip_prefix("/").unwrap();
    }
    final_path.push(target_path);

    Ok(final_path)
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
