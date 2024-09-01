use std::{
    io::{BufRead, BufReader, Error, ErrorKind, Read, Write},
    net::TcpStream,
};

use crate::{
    request::{HttpMethod, HttpRequest},
    response::HttpResponse,
};

pub struct HttpConnection {
    socket: BufReader<TcpStream>,
}

impl HttpConnection {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            socket: BufReader::new(socket),
        }
    }

    pub fn get_next_request(&mut self) -> Result<HttpRequest, std::io::Error> {
        let next_line = read_to_crlf(&mut self.socket)?;
        let (method, target) = parse_request_line(&next_line)?;
        let mut request = HttpRequest::from_request_line(method, target);

        // parse headers
        loop {
            let next_line = read_to_crlf(&mut self.socket)?;
            if next_line == "\r\n" {
                break;
            }

            let (name, value) = match next_line.split_once(":") {
                Some((name, value)) => Ok((name, value)),
                None => Err(Error::new(
                    ErrorKind::InvalidData,
                    "Malformed request headers",
                )),
            }?;

            request.add_header(name, value);

            // if the request has been going on too long, time out
            // -> actual connection timeout should not be handled here, but
            // we should check for request too large. IIRC theres an http
            // response code for that
        }

        Ok(request)
    }

    pub fn send_response(&mut self, response: HttpResponse) -> Result<(), std::io::Error> {
        self.socket
            .get_mut()
            .write_all(response.serialize().as_bytes())?;
        self.socket.get_mut().flush()
    }
}

fn read_to_crlf(socket: &mut BufReader<TcpStream>) -> Result<String, std::io::Error> {
    let mut internal_read_buf = String::new();

    loop {
        socket.take(4096).read_line(&mut internal_read_buf)?;

        if internal_read_buf.ends_with("\r\n") {
            break;
        }

        // if the internal buf gets too long, throw an error
    }

    Ok(internal_read_buf)
}

fn parse_request_line(line: &str) -> Result<(HttpMethod, &str), std::io::Error> {
    let parts = line.trim_end().split(" ").collect::<Vec<&str>>();
    if parts.len() != 3 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Malformed request line",
        ));
    }

    match parts[2] {
        "HTTP/1.1" => Ok(()),
        _ => Err(Error::new(
            ErrorKind::Unsupported,
            "Unsupported HTTP version",
        )),
    }?;

    let method = match parts[0] {
        "GET" => Ok(HttpMethod::GET),
        _ => Err(Error::new(
            ErrorKind::Unsupported,
            "Unsupported HTTP method",
        )),
    }?;

    Ok((method, parts[1]))
}

#[test]
fn test_invalid_request_header() {
    let result = parse_request_line("GET / / HTTP/1.1");
    assert!(result.is_err());
}

#[test]
fn test_ok_request_header() {
    let result = parse_request_line("GET / HTTP/1.1");
    let (method, target) = result.unwrap();

    assert_eq!(method, HttpMethod::GET);
    assert_eq!(target, "/");
}
