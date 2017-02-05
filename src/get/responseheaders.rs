use std::collections::HashMap;
use std::convert::From;

use hyper::header::{ContentType, ContentLength, Headers};
use hyper::server::Response;
use itertools::Itertools;
use serde_json::to_string_pretty;

use makeresponse::MakeResponse;

#[derive(Serialize)]
pub struct ResponseHeaders(HashMap<String, String>);

impl<'a> From<&'a Headers> for ResponseHeaders {
    fn from(headers: &Headers) -> Self {
        ResponseHeaders(headers.iter()
            .map(|h| (h.name().to_string(), h.value_string()))
            .collect::<HashMap<_, _>>())
    }
}

impl MakeResponse for ResponseHeaders {
    fn content_type(&self) -> ContentType {
        ContentType::json()
    }

    fn make_response(&self, mut response: Response) {
        let pretty = |r: &Response| {
            to_string_pretty(&ResponseHeaders::from(r.headers())).unwrap_or_else(|_| String::new())
        };

        *response.status_mut() = self.status();

        for (name, value) in &self.0 {
            response.headers_mut().set_raw(name.clone(), vec![value.clone().into_bytes()]);
        }
        response.headers_mut().set(self.content_type());

        loop {
            let len1 = pretty(&response).len();
            response.headers_mut().set(ContentLength(len1 as u64));
            let len2 = pretty(&response).len();
            if len1 == len2 {
                break;
            }
        }

        let body = pretty(&response);

        response.send(body.as_bytes()).unwrap();
    }
}

pub fn response_headers(path: &str) -> ResponseHeaders {
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
