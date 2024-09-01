use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum HttpStatus {
    OK,
}

impl fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpStatus::OK => write!(f, "200 OK"),
        }
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    pub response_code: HttpStatus,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl HttpResponse {
    pub fn with_status(status: HttpStatus) -> Self {
        Self {
            response_code: status,
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut out_buf = Vec::<u8>::new();
        out_buf.extend("HTTP/1.1 ".as_bytes());
        out_buf.extend(self.response_code.to_string().as_bytes());
        out_buf.extend("\r\n".as_bytes());
        for (name, value) in self.headers.iter() {
            out_buf.extend(name.as_bytes());
            out_buf.extend(": ".as_bytes());
            out_buf.extend(value.as_bytes());
            out_buf.extend("\r\n".as_bytes());
        }
        out_buf.extend("\r\n".as_bytes());
        match &self.body {
            Some(body) => out_buf.extend(body),
            None => {}
        }

        out_buf
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn put_body(&mut self, body: Vec<u8>) {
        self.body = Some(body);
    }
}
