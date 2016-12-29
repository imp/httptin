extern crate hyper;

mod get;
mod post;

use hyper::{Get, Head, Post, Delete};
use hyper::server::{Handler, Server, Request, Response};
use hyper::status::StatusCode;

struct HttpTin;

impl Handler for HttpTin {
    fn handle(&self, request: Request, mut response: Response) {
        println!("{} {} {}", request.remote_addr, request.method, request.uri);
        match request.method {
            Get => get::get(request, response),
            Post => post::post(request, response),
             _ => *response.status_mut() = StatusCode::MethodNotAllowed,
        }
    }
}

fn main() {
    let server = Server::http("::1:8000").unwrap();
    // println!("Server {:?}", server);
    let active = server.handle(HttpTin {}).unwrap();
    println!("Active {:?}", active.socket);
}
