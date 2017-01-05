use std::io::Write;
use hyper::header::{ContentLength, ContentType};
use hyper::server::{Request, Response};
use hyper::status::StatusCode;
use hyper::uri::RequestUri;

macro_rules! dispatch {
    ($m0:expr => $h0:expr, $($m1:expr => $h1:expr,)*) => {{
        if $m0 { $h0 } $( else if $m1 { $h1 } )*
    }}
}

trait MakeResponse {
    fn status(&self) -> StatusCode {
        StatusCode::Ok
    }

    fn content_type(&self) -> ContentType {
        ContentType::plaintext()
    }

    fn content_length(&self) -> ContentLength {
        ContentLength(0)
    }

    fn content(&self) -> &[u8] {
        &[]
    }

    fn make_response(&self, mut response: Response) {
        *response.status_mut() = self.status();
        response.headers_mut().set(self.content_type());
        response.headers_mut().set(self.content_length());
        response.start().unwrap().write(self.content()).unwrap();
    }
}

impl MakeResponse for StatusCode {
    fn status(&self) -> StatusCode {
        *self
    }
}

impl MakeResponse for String {
    fn content_type(&self) -> ContentType {
        ContentType::html()
    }

    fn content_length(&self) -> ContentLength {
        ContentLength(self.as_bytes().len() as u64)
    }

    fn content(&self) -> &[u8] {
        self.as_bytes()
    }
}

pub fn get(request: Request, response: Response) {
    println!("** Handling GET {}", request.uri);
    println!("** Incoming headers {:?}", request.headers);
    if let RequestUri::AbsolutePath(ref path) = request.uri {
        dispatch![
            path == "/" => index().make_response(response),
            path == "/ip" => origin(&request).make_response(response),
            path.starts_with("/status/") => status(path).make_response(response),
            path.starts_with("/test") => test(&request).make_response(response),
            true => notfound404().make_response(response),
        ];
    }
}

struct Text(String);

impl MakeResponse for Text {
    fn content_type(&self) -> ContentType {
        ContentType::plaintext()
    }

    fn content_length(&self) -> ContentLength {
        ContentLength(self.0.as_bytes().len() as u64)
    }

    fn content(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

fn notfound404() -> String {
    String::from("<!DOCTYPE html>
    <html>
        <head>
            <title>404 Not Found</title>
        </head>
        <body>
        <h1>Not Found</h1>
        <p>The requested URL was not found on the server.
        If you entered the URL manually please check your spelling and try again.</p>
        </body>
    </html>")
}

fn index() -> String {
    String::from("<!DOCTYPE html>
    <html>
        <head>
            <link
                rel=\"icon\"
                href=\"http://investorintel.com/wp-content/uploads/2014/03/tin-can.jpg\"/>
            <title>HTTPTIN</title>
        </head>
        <body>
        <h1>HTTPTIN - HTTP tester in Rust and Rocket</h1>
        </body>
    </html>")
}

fn status(path: &str) -> StatusCode {
    // /status/xx
    // 0123456789
    let (_, param) = path.split_at(8);
    match param.parse::<u16>() {
        Ok(status) => StatusCode::from_u16(status),
        Err(_) => StatusCode::BadRequest,
    }
}

fn origin(request: &Request) -> Text {
    Text(format!("origin: {}", request.remote_addr))
}

fn test(request: &Request) -> String {
    format!("<!DOCTYPE html>
    <html>
        <head>
            <title>HTTPTIN TEST</title>
        </head>
        <body>
            Remote Address: {}<br>
            Method: {}<br>
            HTTPVersion: {}<br>
            Headers: {}<br>
            URI: {}<br>
        </body>
    </html>",
            request.remote_addr,
            request.method,
            request.version,
            request.headers,
            request.uri)
}
