use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    HEAD,
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub target: String,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl HttpRequest {
    pub fn from_request_line(method: HttpMethod, target: &str) -> Self {
        Self {
            method,
            target: String::from(target),
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn read_header(&self, key: &str) -> Option<&str> {
        self.headers.get(key).map(|x| x.as_str())
    }
}
