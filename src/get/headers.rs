use std::collections::HashMap;

use hyper::header::{ContentType, Headers};
use hyper::server::{Request, Response};
use hyper_serde::serialize_pretty;
use serde_json::to_string_pretty;

use makeresponse::MakeResponse;

#[derive(Serialize)]
pub struct NativeHeadersData {
    #[serde(serialize_with="serialize_pretty")]
    headers: Headers,
}

impl MakeResponse for NativeHeadersData {
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

#[derive(Serialize)]
pub struct HeadersData {
    headers: HashMap<String, String>,
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

pub fn headers(request: &Request) -> HeadersData {
    // NativeHeadersData { headers: request.headers.clone() }
    let headers = request.headers
        .iter()
        .map(|h| (h.name().to_string(), h.value_string()))
        .collect::<HashMap<_, _>>();
    HeadersData { headers: headers }
}
