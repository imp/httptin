use makeresponse::Html;

const INDEX: &'static str = include_str!("index.html");

pub fn index() -> Html {
    Html(String::from(INDEX))
}
