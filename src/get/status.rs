use hyper::StatusCode;
use hyper::status::InvalidStatusCode;

use makeresponse::MakeResponse;

impl MakeResponse for StatusCode {
    fn status(&self) -> StatusCode {
        *self
    }
}

pub fn status(path: &str) -> StatusCode {
    path.trim_left_matches("/status")
        .parse::<u16>()
        .map_err(|_| InvalidStatusCode)
        .and_then(|c| StatusCode::try_from(c))
        .unwrap_or(StatusCode::BadRequest)
}
