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

    pub fn serialize(&self) -> String {
        let mut out_buf = String::new();
        out_buf += "HTTP/1.1 ";
        out_buf += self.response_code.to_string().as_str();
        out_buf += "\r\n";
        for (name, value) in self.headers.iter() {
            out_buf += name;
            out_buf += ": ";
            out_buf += value;
            out_buf += "\r\n";
        }
        out_buf += "\r\n";

        out_buf
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }
}
