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
