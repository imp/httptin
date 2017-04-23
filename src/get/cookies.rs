use std::collections::HashMap;

use hyper::header::{ContentType, Cookie};
use hyper::server::{Request, Response};
use itertools::Itertools;
use serde_json::to_string_pretty;

use makeresponse::MakeResponse;

#[derive(Serialize)]
pub struct Cookies(HashMap<String, String>);

impl Cookies {
    pub fn from_request(request: &Request) -> Self {
        let cookies = request
            .headers
            .get::<Cookie>()
            .iter()
            .map(|c| c.0.iter())
            .flatten()
            .map(|s| s.splitn(2, '=').tuples())
            .flatten()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<HashMap<_, _>>();
        Cookies(cookies)
    }
}

impl MakeResponse for Cookies {
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
