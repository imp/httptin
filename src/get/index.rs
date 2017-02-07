use makeresponse::Html;

const INDEX: &'static str =
"<!DOCTYPE html>
<html>
    <head>
        <title>HTTPTIN</title>
    </head>
    <body>
    <h1>HTTPTIN - HTTP tester in Rust and Rocket</h1>
    </body>
</html>";

pub fn index() -> Html {
    Html(String::from(INDEX))
}
