use std::collections::HashMap;

use hyper::header::ContentType;
use hyper::server::{Request, Response};
use serde_json::to_string_pretty;

use makeresponse::MakeResponse;

#[derive(Serialize)]
pub struct HeadersData(HashMap<String, String>);

impl HeadersData {
    pub fn from_request(request: &Request) -> Self {
        let headers = request
            .headers
            .iter()
            .map(|h| (h.name().to_string(), h.value_string()))
            .collect::<HashMap<_, _>>();
        HeadersData(headers)
    }
}

impl MakeResponse for HeadersData {
    fn content_type(&self) -> ContentType {
        ContentType::json()
    }

    fn make_response(&self, mut response: Response) {
        *response.status_mut() = self.status();
        response.headers_mut().set(self.content_type());
        let body = to_string_pretty(self).unwrap_or_else(|_| String::new());
        response.send(body.as_bytes()).unwrap();
    }
}
