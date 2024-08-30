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
}

impl HttpResponse {
    pub fn with_status(status: HttpStatus) -> Self {
        Self {
            response_code: status,
            headers: HashMap::new(),
        }
    }

    pub fn serialize(&self) -> String {
        let mut end_value = String::new();
        end_value += "HTTP/1.1 ";
        end_value += self.response_code.to_string().as_str();
        end_value += "\r\n";
        for (name, value) in self.headers.iter() {
            end_value += name;
            end_value += ": ";
            end_value += value;
            end_value += "\r\n";
        }
        end_value += "\r\n";

        end_value
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }
}
