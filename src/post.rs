use hyper::server::{Request, Response};

pub fn post(request: Request, mut response: Response) {
    println!("** Handling POST {} from {}", request.uri, request.remote_addr);
    println!("** Incoming headers {:?}", request.headers);
}
