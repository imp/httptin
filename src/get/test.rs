use hyper::server::Request;

use makeresponse::Html;

pub fn test(request: &Request) -> Html {
    Html(format!(
        "<!DOCTYPE html>
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
        request.uri
    ))
}
