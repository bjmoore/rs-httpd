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
    pub headers: HashMap<String, String>, // this should be private
}
