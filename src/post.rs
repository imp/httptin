use hyper::server::{Request, Response};

pub fn post(request: Request, mut response: Response) {
    let (remote_addr, _, headers, uri, version, payload) = request.deconstruct();
}
