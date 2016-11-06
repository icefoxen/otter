#[macro_use]
extern crate nickel;

use nickel::{Nickel, HttpRouter};


fn get_page(request: &mut nickel::Request) -> String {

    "Get page!".to_owned()
}


fn post_page(request: &mut nickel::Request) -> String {

    "Post page!".to_owned()
}

// fn test(request: &mut nickel::Request) -> String {
//     "Test string!".to_owned()
// }

fn main() {
    let mut server = Nickel::new();
    // server.utilize(nickel::StaticFilesHandler::new("htdocs"));
    server.get("/*", middleware!(|request| get_page(request).as_str()))
          .post("/*", middleware!(|request| post_page(request).as_str()));
    server.listen("127.0.0.1:8080").unwrap();
}
