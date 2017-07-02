use hyper::server::{Request, Response};
use slog::Logger;

pub fn handler(logger: &Logger, request: &Request, mut response: Response) {
    let raddr = request.remote_addr().unwrap();
    info!(
        logger,
        "** Handling POST {} from {}",
        request.uri(),
        raddr
    );
    info!(logger, "** Incoming headers {:?}", request.headers());
}
