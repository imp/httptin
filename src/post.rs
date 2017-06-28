use hyper::server::{Request, Response};
use slog::Logger;

pub fn handler(logger: &Logger, request: &Request, mut response: Response) {
    info!(
        logger,
        "** Handling POST {} from {}",
        request.uri,
        request.remote_addr
    );
    info!(logger, "** Incoming headers {:?}", request.headers);
}
