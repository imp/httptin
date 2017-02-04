use hyper::server::{Request, Response};
use hyper::uri::RequestUri;
use itertools::Itertools;
use serde_json::{Map, Value};
use serde_json::to_value;

use makeresponse::{Html, MakeResponse, ResponseHeaders};

mod headers;
mod status;

macro_rules! dispatch {
    ($m0:expr => $h0:expr, $($m1:expr => $h1:expr,)*) => {{
        if $m0 { $h0 } $( else if $m1 { $h1 } )*
    }}
}

pub fn handler(request: Request, response: Response) {
    println!("** Handling GET {}", request.uri);
    println!("** Incoming headers {:?}", request.headers);
    if let RequestUri::AbsolutePath(ref path) = request.uri {
        dispatch![
            path == "/" => index().make_response(response),
            path == "/ip" => origin(&request).make_response(response),
            path == "/headers" => headers::headers(&request).make_response(response),
            path.starts_with("/get") => get(&request).make_response(response),
            path.starts_with("/status/") => status::status(path).make_response(response),
            path.starts_with("/response-headers") => response_headers(path).make_response(response),
            path.starts_with("/test") => test(&request).make_response(response),
            true => notfound404().make_response(response),
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
            <link
                rel=\"icon\"
                href=\"http://investorintel.com/wp-content/uploads/2014/03/tin-can.jpg\"/>
            <title>HTTPTIN</title>
        </head>
        <body>
        <h1>HTTPTIN - HTTP tester in Rust and Rocket</h1>
        </body>
    </html>"))
}

fn get(request: &Request) -> Value {
    let mut map = Map::new();
    let headers = request.headers
        .iter()
        .map(|h| (String::from(h.name()), Value::String(h.value_string())))
        .collect::<Map<_, _>>();
    //let headers = serialize_pretty(request.headers).unwrap();
    map.insert(String::from("headers"), Value::Object(headers));
    map.insert(String::from("origin"), origin(request));

    Value::Object(map)
}

fn origin(request: &Request) -> Value {
    let mut map = Map::new();
    let ip = Value::String(format!("{}", request.remote_addr.ip()));
    let port = Value::String(format!("{}", request.remote_addr.port()));
    let ipv4 = Value::Bool(request.remote_addr.is_ipv4());
    let ipv6 = Value::Bool(request.remote_addr.is_ipv6());
    map.insert(String::from("ip"), ip);
    map.insert(String::from("port"), port);
    map.insert(String::from("ipv4"), ipv4);
    map.insert(String::from("ipv6"), ipv6);

    Value::Object(map)
}

fn response_headers(path: &str) -> ResponseHeaders {
    // /response-headers?header1=value&header2=value
    let headers = path.trim_left_matches("/response-headers")
        .trim_left_matches('?')
        .split('&')
        .map(|i| i.splitn(2, '=').tuples())
        .flatten()
        .map(|(i, j)| (String::from(i), to_value(j).unwrap()))
        .collect::<Map<_, _>>();

    ResponseHeaders(headers)
}

fn test(request: &Request) -> Html {
    Html(format!("<!DOCTYPE html>
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
                 request.uri))
}