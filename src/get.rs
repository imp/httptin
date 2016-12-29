use hyper::server::{Request, Response};

pub fn get(request: Request, mut response: Response) {
    let (remote_addr, _, headers, uri, version, payload) = request.deconstruct();
    println!("**  Handling GET {}", uri);
    println!("*** Incoming headers {:?}", headers);
}

// // #[get("/")]
// fn index() -> content::HTML<&'static str> {
//     content::HTML("<!DOCTYPE html>
//     <html>
//         <head>
//             <title>HTTPTIN</title>
//         </head>
//         <body>
//         <h1>HTTPTIN - HTTP tester in Rust and Rocket</h1>
//         </body>
//     </html>")
// }
//
// // #[get("/test")]
// fn test(data: IndexData) -> content::HTML<String> {
//     content::HTML(format!("<!DOCTYPE html>
//     <html>
//         <head>
//             <title>HTTPTIN></title>
//         </head>
//         <body>{}</body>
//     </html>",
//                           data))
// }

//#[get("/ip")]
fn origin() -> String {
    String::from("Remote address decoding is not implemented yet")
}

struct GetData {
    args: String,
    headers: String,
}

// //#[get("/status/<code>")]
// pub fn status<'r>(code: u16) -> Response<'r> {
//     Response::build().status(Status::raw(code)).finalize()
// }
