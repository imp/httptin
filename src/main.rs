extern crate hyper;

mod get;
mod post;

use hyper::{Get, Post};
use hyper::header;
use hyper::server::{Handler, Server, Request, Response};
use hyper::status::StatusCode;

struct HttpTin {
    server: String,
}

impl HttpTin {
    pub fn new() -> Self {
        let server = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        HttpTin { server: server }
    }

    fn prepare_response(&self, response: &mut Response) {
        let server = header::Server(self.server.clone());
        response.headers_mut().set(server);
    }
}

impl Handler for HttpTin {
    fn handle(&self, request: Request, mut response: Response) {
        println!("{} {} {}", request.remote_addr, request.method, request.uri);
        self.prepare_response(&mut response);
        match request.method {
            Get => get::get(request, response),
            Post => post::post(request, response),
            _ => *response.status_mut() = StatusCode::MethodNotAllowed,
        }
    }
}

fn main() {
    let server = Server::http("::1:8000").unwrap();
    if let Ok(active) = server.handle(HttpTin::new()) {
        println!("Listening on {:?}", active.socket);
    }
}
