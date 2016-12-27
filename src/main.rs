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

#[get("/get")]
fn get() -> String {
    String::from("method")
}

#[get("/status/<status>")]
pub fn status(status: &str) -> String {
    format!("<!DOCTYPE html><html><title>HTTPTIN></title><body>Status: {} (FIX ME)</body></html>", status)
}

fn main() {
    rocket::ignite().mount("/", routes![index, origin, get, status, test]).launch();
}
