extern crate hyper;
extern crate itertools;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate slog;
extern crate slog_term;

mod get;
mod post;
mod makeresponse;

use hyper::{Get, Post};
use hyper::header;
use hyper::server::{Handler, Server, Request, Response};
use hyper::status::StatusCode;
use slog::DrainExt;

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

    fn prepare_response(&self, response: &mut Response) {
        let server = header::Server(self.server.clone());
        response.headers_mut().set(server);
    }
}

impl Handler for HttpTin {
    fn handle(&self, request: Request, mut response: Response) {
        let logger = self.logger
            .new(o!("peer" => format!("{}", request.remote_addr)));
        self.prepare_response(&mut response);
        match request.method {
            Get => get::handler(logger, request, response),
            Post => post::handler(logger, request, response),
            _ => *response.status_mut() = StatusCode::MethodNotAllowed,
        }
    }
}

fn main() {
    let drain = slog_term::streamer().compact().build().fuse();
    let logger = slog::Logger::root(drain, o!());
    let httptin = HttpTin::new(&logger);
    let server = Server::http("localhost:8000").unwrap();
    if let Ok(active) = server.handle(httptin) {
        info!(logger, "{:?}", active);
    }
}
