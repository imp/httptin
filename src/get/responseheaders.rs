use std::collections::HashMap;

use hyper::header::{ContentType, ContentLength, Headers};
use hyper::server::Response;
use itertools::Itertools;
use serde_json::to_string_pretty;

use makeresponse::MakeResponse;

#[derive(Serialize)]
pub struct ResponseHeaders(HashMap<String, String>);

impl ResponseHeaders {
    fn from_headers(headers: &Headers) -> Self {
        ResponseHeaders(headers.iter()
            .map(|h| (h.name().to_string(), h.value_string()))
            .collect::<HashMap<_, _>>())
    }

    pub fn from_path(path: &str) -> Self {
        // /response-headers?header1=value&header2=value
        let headers = path.trim_left_matches("/response-headers")
            .trim_left_matches('?')
            .split('&')
            .map(|i| i.splitn(2, '=').tuples())
            .flatten()
            .map(|(i, j)| (i.to_string(), j.to_string()))
            .collect::<HashMap<_, _>>();

        ResponseHeaders(headers)
    }
}

impl MakeResponse for ResponseHeaders {
    fn content_type(&self) -> ContentType {
        ContentType::json()
    }

    fn make_response(&self, mut response: Response) {
        let pretty = |h: &Headers| {
            to_string_pretty(&ResponseHeaders::from_headers(h)).unwrap_or_else(|_| String::new())
        };

        *response.status_mut() = self.status();

        for (name, value) in &self.0 {
            response.headers_mut().set_raw(name.clone(), vec![value.clone().into_bytes()]);
        }
        response.headers_mut().set(self.content_type());

        loop {
            let len1 = pretty(response.headers()).len();
            response.headers_mut().set(ContentLength(len1 as u64));
            let len2 = pretty(response.headers()).len();
            if len1 == len2 {
                break;
            }
        }

        let body = pretty(response.headers());

        response.send(body.as_bytes()).unwrap();
    }
}
