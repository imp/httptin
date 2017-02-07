use hyper::server::{Request, Response};
use hyper::uri::RequestUri;

use makeresponse::{Html, MakeResponse};

mod favicon;
mod getdata;
mod headers;
mod origin;
mod status;
mod responseheaders;
mod test;

use self::favicon::Favicon;
use self::getdata::GetData;
use self::headers::HeadersData;
use self::origin::Origin;
use self::responseheaders::ResponseHeaders;

macro_rules! dispatch {
    ($r:expr, $m0:expr => $h0:expr, $($m1:expr => $h1:expr,)*) => {{
        if $m0 { $h0.make_response($r) } $( else if $m1 { $h1.make_response($r) } )*
    }}
}

pub fn handler(request: Request, response: Response) {
    println!("** Handling GET {}", request.uri);
    // println!("** Incoming headers {:?}", request.headers);
    if let RequestUri::AbsolutePath(ref path) = request.uri {
        dispatch![
            response,
            path == "/" => index(),
            path == "/ip" => Origin::from_request(&request),
            path == "/headers" => HeadersData::from_request(&request),
            path == "/favicon.ico" => Favicon::default(),
            path.starts_with("/get") => GetData::from(&request),
            path.starts_with("/status/") => status::status(path),
            path.starts_with("/response-headers") => ResponseHeaders::from_path(path),
            path.starts_with("/test") => test::test(&request),
            true => notfound404(),
        ];
    }
}

fn notfound404() -> Html {
    Html(String::from("<!DOCTYPE html>
    <html>
        <head>
            <title>404 Not Found</title>
        </head>
        <body>
        <h1>Not Found</h1>
        <p>The requested URL was not found on the server.
        If you entered the URL manually please check your spelling and try again.</p>
        </body>
    </html>"))
}

fn index() -> Html {
    Html(String::from("<!DOCTYPE html>
    <html>
        <head>
            <title>HTTPTIN</title>
        </head>
        <body>
        <h1>HTTPTIN - HTTP tester in Rust and Rocket</h1>
        </body>
    </html>"))
}
