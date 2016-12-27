#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::fmt;

use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use rocket::response::content;
use rocket::{Outcome, Response};

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
fn index() -> content::HTML<&'static str> {
    content::HTML("<!DOCTYPE html>
    <html>
        <head>
            <title>HTTPTIN</title>
        </head>
        <body>
        <h1>HTTPTIN - HTTP tester in Rust and Rocket</h1>
        </body>
    </html>")
}

#[get("/test")]
fn test(data: IndexData) -> String {
    format!("<!DOCTYPE html><html><head><title>HTTPTIN></title></head><body>{}</body></html>", data)
}

#[get("/ip")]
fn origin() -> String {
    String::from("Remote address decoding is not implemented yet")
}

#[derive(Serialize)]
struct GetData {
    args: String,
    headers: String,
}

#[get("/get")]
fn get() -> content::JSON<&'static str> {
    content::JSON("{agrs: {}}")
}

#[get("/status/<code>")]
pub fn status<'r>(code: u16) -> Response<'r> {
    Response::build().status(Status::raw(code)).finalize()
}

fn main() {
    rocket::ignite().mount("/", routes![index, origin, get, status, test]).launch();
}
