use hyper::StatusCode;

use makeresponse::MakeResponse;

impl MakeResponse for StatusCode {
    fn status(&self) -> StatusCode {
        *self
    }
}

pub fn status(path: &str) -> StatusCode {
    match path.trim_left_matches("/status/").parse::<u16>() {
        Ok(status) => StatusCode::from_u16(status),
        Err(_) => StatusCode::BadRequest,
    }
}
