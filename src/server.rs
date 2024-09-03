use std::io::{Error, ErrorKind};
use std::path::Path;
use std::path::PathBuf;
use tokio::net::{TcpListener, TcpStream};

use crate::response::HttpStatus;
use crate::{connection::HttpConnection, request::HttpRequest, response::HttpResponse};

pub struct HttpServer {
    address: String,
}

impl HttpServer {
    pub fn new(address: &str) -> std::io::Result<Self> {
        Ok(Self {
            address: String::from(address),
        })
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(&self.address).await?;
        loop {
            let (connection, addr) = listener.accept().await?;
            tokio::spawn(handle_client(connection));
        }
    }
}

async fn handle_client(stream: TcpStream) -> std::io::Result<()> {
    let mut conn = HttpConnection::new(stream);
    // In H2 this will be getting the next frame and routing it to a new greenthread
    let request = conn.get_next_request().await?;
    // In H2/3 I think we need:
    // reader task peels the next frame out of the socket
    // writer task gets an mpsc receiver
    // for a new stream, spawn a new task and put it into a map
    // each stream task has a cloned mpsc sender to send frames to the writer task
    let response = generate_response(request)?;
    conn.send_response(response).await?;
    Ok(())
}

fn generate_response(request: HttpRequest) -> Result<HttpResponse, std::io::Error> {
    let mut response = HttpResponse::with_status(HttpStatus::Ok);
    response.add_header("Content-Length", "0");

    validate_request(&request)?;

    // are we trying to get a file?
    // are we allowed to get the file?
    let path = route_request_to_local_fs(&request)?;

    // add a lot of error handling here
    println!("trying to read: {}", path.display());
    let file_contents = std::fs::read(path);
    match &file_contents {
        Err(e) => match e.kind() {
            ErrorKind::NotFound => response.response_code = HttpStatus::NotFound,
            _ => response.response_code = HttpStatus::InternalServerError,
        },
        Ok(contents) => {
            let body_size = contents.len();
            response.add_header("Content-Length", body_size.to_string().as_str());
            response.add_header("Connection", "close");
            response.put_body(contents.to_vec());
        }
    };

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
