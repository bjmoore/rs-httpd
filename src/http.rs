use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

pub enum HttpMethod {
    GET,
    POST,
    HEAD,
}

pub struct HttpRequest {
    method: HttpMethod,
}

pub struct HttpResponse {
    pub response_code: u8,
}

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
        let mut request = HttpRequest {
            method: HttpMethod::GET,
        };

        let next_line = read_to_crlf(&mut self.socket)?;

        parse_httpreq_header(&next_line);

        // parse headers
        loop {
            let next_line = read_to_crlf(&mut self.socket)?;

            if next_line == "\r\n" {
                break;
            }

            // if the request has been going on too long, time out
            // -> actual connection timeout should not be handled here, but
            // we should check for request too large. IIRC theres an http
            // response code for that
        }

        Ok(request)
    }

    pub fn send_reply(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}

fn read_to_crlf(socket: &mut BufReader<TcpStream>) -> Result<String, std::io::Error> {
    let mut internal_read_buf = String::new();

    loop {
        socket.read_line(&mut internal_read_buf)?;

        if internal_read_buf.ends_with("\r\n") {
            break;
        }

        // if the internal buf gets too long, throw an error
    }

    Ok(internal_read_buf)
}

fn parse_httpreq_header(line: &str) {
    // GEt / HTTP/1.1
    // return -> method, target, valid 1.1
}
