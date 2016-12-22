extern crate pencil;

use std::collections::BTreeMap;
use pencil::Pencil;
use pencil::{Request, PencilResult, Response, jsonify};
use pencil::method::Get;

fn index(req: &mut Request) -> PencilResult {
    let text = format!("<!doctype html><html><title>HTTPTIN></title><body>{:#?} {:?}</body></html>",
                       req.remote_addr,
                       req.headers);
    Ok(Response::from(text))
}

fn get(req: &mut Request) -> PencilResult {
    let mut data = BTreeMap::new();
    // data.insert("args", format!("{:?}", req.args()));
    data.insert("ip", format!("{}", req.remote_addr().ip()));
    if let Some(endpoint) = req.endpoint() {
        data.insert("endpoint", endpoint);
    }
    data.insert("url", req.url());
    jsonify(&data)
}

fn main() {
    let addr = "127.0.0.1:5000";
    let mut app = Pencil::new("httptin");
    app.route("/", &[Get], "index", index);
    app.route("/get", &[Get], "get", get);
    println!("Listening on {}", addr);
    app.run(addr);
}
