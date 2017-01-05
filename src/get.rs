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

trait ToResponse {
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
}

impl ToResponse for StatusCode {
    fn status(&self) -> StatusCode {
        *self
    }
}

impl ToResponse for String {
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

pub fn get(request: Request, mut response: Response) {
    println!("** Handling GET {}", request.uri);
    println!("** Incoming headers {:?}", request.headers);
    if let RequestUri::AbsolutePath(ref path) = request.uri {
        dispatch![
            path == "/" => index(&request, response),
            path == "/ip" => origin(&request, response),
            path.starts_with("/status/") => status(path, response),
            path.starts_with("/test") => test(&request, response),
            true => notfound404(response),
        ];
    }
}

fn notfound404(mut response: Response) {
    let text = "<!DOCTYPE html>
    <html>
        <head>
            <title>404 Not Found</title>
        </head>
        <body>
        <h1>Not Found</h1>
        <p>The requested URL was not found on the server.
        If you entered the URL manually please check your spelling and try again.</p>
        </body>
    </html>";

    response.headers_mut().set(ContentType::html());
    response.headers_mut().set(ContentLength(text.len() as u64));
    response.start().unwrap().write(text.as_bytes()).unwrap();
}

fn index(request: &Request, mut response: Response) {
    let index = "<!DOCTYPE html>
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
    </html>";

    response.headers_mut().set(ContentType::html());
    response.headers_mut().set(ContentLength(index.len() as u64));
    response.start().unwrap().write(index.as_bytes()).unwrap();
}

fn status(path: &str, mut response: Response) {
    // /status/xx
    // 0123456789
    let (_, param) = path.split_at(8);
    let code = match param.parse::<u16>() {
        Ok(status) => StatusCode::from_u16(status),
        Err(_) => StatusCode::BadRequest,
    };
    *response.status_mut() = code;
}

fn origin(request: &Request, mut response: Response) {
    let text = "Remote address decoding is not implemented yet";
    response.headers_mut().set(ContentType::plaintext());
    response.headers_mut().set(ContentLength(text.len() as u64));
    response.start().unwrap().write(text.as_bytes()).unwrap();
}

fn test(request: &Request, mut response: Response) {
    let text = format!("<!DOCTYPE html>
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
                       request.uri);
    response.headers_mut().set(ContentType::html());
    response.headers_mut().set(ContentLength(text.len() as u64));
    response.start().unwrap().write(text.as_bytes()).unwrap();
}
