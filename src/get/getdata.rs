use hyper::header::ContentType;
use hyper::server::{Request, Response};
use serde_json::to_string_pretty;

use makeresponse::MakeResponse;
use super::headers::HeadersData;
use super::origin::Origin;

#[derive(Serialize)]
pub struct GetData {
    headers: HeadersData,
    origin: Origin,
}

impl GetData {
    pub fn from(request: &Request) -> Self {
        let headers = HeadersData::from_request(request);
        let origin = Origin::from_request(request);
        GetData {
            headers: headers,
            origin: origin,
        }
    }
}

impl MakeResponse for GetData {
    fn content_type(&self) -> ContentType {
        ContentType::json()
    }

    fn make_response(&self, mut response: Response) {
        response.set_status(self.status());
        response.headers_mut().set(self.content_type());
        let body = to_string_pretty(self).unwrap_or_else(|_| String::new());
        response.send(body.as_bytes()).unwrap();
    }
}
