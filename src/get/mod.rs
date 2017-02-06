use hyper::server::{Request, Response};
use hyper::uri::RequestUri;

use makeresponse::{Html, MakeResponse};

mod getdata;
mod headers;
mod origin;
mod status;
mod responseheaders;
mod test;

use self::getdata::GetData;
use self::headers::HeadersData;
use self::origin::Origin;
use self::responseheaders::ResponseHeaders;

macro_rules! dispatch {
    ($m0:expr => $h0:expr, $($m1:expr => $h1:expr,)*) => {{
        if $m0 { $h0 } $( else if $m1 { $h1 } )*
    }}
}

pub fn handler(request: Request, response: Response) {
    println!("** Handling GET {}", request.uri);
    // println!("** Incoming headers {:?}", request.headers);
    if let RequestUri::AbsolutePath(ref path) = request.uri {
        dispatch![
            path == "/" => index().make_response(response),
            path == "/ip" => Origin::from_request(&request).make_response(response),
            path == "/headers" => HeadersData::from_request(&request).make_response(response),
            path.starts_with("/get") => GetData::from(&request).make_response(response),
            path.starts_with("/status/") => status::status(path).make_response(response),
            path.starts_with("/response-headers") => ResponseHeaders::from_path(path).make_response(response),
            path.starts_with("/test") => test::test(&request).make_response(response),
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
