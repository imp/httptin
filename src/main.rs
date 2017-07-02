extern crate futures;
extern crate hyper;
extern crate itertools;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate slog;
extern crate slog_term;

use futures::future;
use hyper::{Get, Post};
use hyper::header;
use hyper::server::{Http, Request, Response, Service};
use hyper::StatusCode;
use slog::DrainExt;

mod get;
mod post;
mod makeresponse;

struct HttpTin {
    server: String,
    logger: slog::Logger,
}

impl HttpTin {
    pub fn new(logger: &slog::Logger) -> Self {
        let server = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        let logger = logger.new(o!("server" => server.clone()));
        HttpTin {
            server: server,
            logger: logger,
        }
    }

    fn prepare_response(&self) -> Response {
        let mut response = Response::new();
        let server = header::Server(self.server.clone());
        response.headers_mut().set(server);
        response
    }
}

impl Service for HttpTin {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, request: Request) -> Self::Future {
        let logger = self.logger
            .new(o!("peer" => format!("{}", request.remote_addr().unwrap())));
        let mut response = self.prepare_response();
        match *request.method() {
            Get => get::handler(&logger, &request, response),
            Post => post::handler(&logger, &request, response),
            _ => response.set_status(StatusCode::MethodNotAllowed),
        }
    }
}

fn main() {
    let drain = slog_term::streamer().compact().build().fuse();
    let logger = slog::Logger::root(drain, o!());
    let httptin = HttpTin::new(&logger);
    let addr = "localhost::8000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(httptin)).unwrap();
    if let Ok(active) = server.run() {
        info!(logger, "{:?}", active);
    }

    // let server = Server::http("localhost:8000").unwrap();
    // if let Ok(active) = server.handle(httptin) {
    //     info!(logger, "{:?}", active);
    // }
}
