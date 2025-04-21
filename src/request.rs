pub struct HttpRequest {
    pub method: String,
    pub path: String,
}

impl HttpRequest {
    pub fn new(method: String, path: String) -> Self {
        Self { method, path }
    }
}