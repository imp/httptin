use hyper::header::{ContentLength, ContentType};
use hyper::server::Response;
use hyper::status::StatusCode;
use serde_json::{to_vec_pretty, Value};

pub trait MakeResponse {
    fn len(&self) -> usize {
        self.content().len()
    }

    fn status(&self) -> StatusCode {
        StatusCode::Ok
    }

    fn content_type(&self) -> ContentType {
        ContentType::plaintext()
    }

    fn content_length(&self) -> ContentLength {
        ContentLength(self.len() as u64)
    }

    fn content(&self) -> &[u8] {
        &[]
    }

    fn make_response(&self, mut response: Response) {
        *response.status_mut() = self.status();
        response.headers_mut().set(self.content_type());
        response.send(self.content()).unwrap();
        // response.headers_mut().set(self.content_length());
        // response.start().unwrap().write(self.content()).unwrap();
    }
}

pub struct Html(pub String);

impl MakeResponse for Html {
    fn content_type(&self) -> ContentType {
        ContentType::html()
    }

    fn content_length(&self) -> ContentLength {
        ContentLength(self.len() as u64)
    }

    fn content(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl MakeResponse for String {
    fn content_type(&self) -> ContentType {
        ContentType::plaintext()
    }

    fn content_length(&self) -> ContentLength {
        ContentLength(self.len() as u64)
    }

    fn content(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl MakeResponse for Value {
    fn content_type(&self) -> ContentType {
        ContentType::json()
    }

    fn len(&self) -> usize {
        to_vec_pretty(self).unwrap_or_else(|_| Vec::new()).len()
    }

    fn make_response(&self, mut response: Response) {
        *response.status_mut() = self.status();
        response.headers_mut().set(self.content_type());
        let body = to_vec_pretty(self).unwrap_or_else(|_| Vec::new());
        response.send(&body).unwrap();
    }
}
