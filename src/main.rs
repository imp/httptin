#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::fmt;

use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};

struct IndexData(String);

impl<'a, 'r> FromRequest<'a, 'r> for IndexData {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<IndexData, ()> {
        Outcome::Success(IndexData(format!("{}", request)))
    }
}

impl fmt::Display for IndexData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[get("/")]
fn index(data: IndexData) -> String {
    format!("<!DOCTYPE html><html><title>HTTPTIN></title><body>{}</body></html>", data)
}

#[get("/ip")]
fn origin() -> String {
    String::from("Remote address decoding is not implemented yet")
}

#[get("/get")]
fn get() -> String {
    String::from("method")
}

fn main() {
    rocket::ignite().mount("/", routes![index, origin, get]).launch();
}

// fn get(req: &mut Request) -> PencilResult {
//     let mut data = BTreeMap::new();
//     // data.insert("args", format!("{:?}", req.args().listiter().collect::<Vec<_>>()));
//     data.insert("ip", format!("{}", req.remote_addr().ip()));
//     if let Some(endpoint) = req.endpoint() {
//         data.insert("endpoint", endpoint);
//     }
//     data.insert("url", req.url());
//
//     let mut args = BTreeMap::new();
//     for (key, value) in req.args().listiter() {
//         args.insert(key, format!("{:?}", value));
//     }
//     data.insert("args", format!("{:?}", args));
//     jsonify(&data)
// }
