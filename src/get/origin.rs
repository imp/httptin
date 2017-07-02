use std::net::IpAddr;

use hyper::header::ContentType;
use hyper::server::{Request, Response};
use serde_json::to_string_pretty;

use makeresponse::MakeResponse;

#[derive(Serialize)]
pub struct Origin {
    ip: IpAddr,
    port: u16,
    ipv4: bool,
    ipv6: bool,
}

impl Origin {
    pub fn from_request(request: &Request) -> Self {
        let addr = request.remote_addr().unwrap();
        Origin {
            ip: addr.ip(),
            port: addr.port(),
            ipv4: addr.is_ipv4(),
            ipv6: addr.is_ipv6(),
        }
    }
}

impl MakeResponse for Origin {
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
