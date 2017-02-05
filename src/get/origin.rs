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

impl MakeResponse for Origin {
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

pub fn origin(request: &Request) -> Origin {
    Origin {
        ip: request.remote_addr.ip(),
        port: request.remote_addr.port(),
        ipv4: request.remote_addr.is_ipv4(),
        ipv6: request.remote_addr.is_ipv6(),
    }
}
