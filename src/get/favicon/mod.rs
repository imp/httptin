use hyper::header::ContentType;
use hyper::server::Response;

use makeresponse::MakeResponse;

#[derive(Default)]
pub struct Favicon;

const FAVICON: &'static [u8] = include_bytes!("tin-can.png");

impl MakeResponse for Favicon {
    fn content_type(&self) -> ContentType {
        ContentType::png()
    }

    fn content(&self) -> &[u8] {
        FAVICON
    }
}
