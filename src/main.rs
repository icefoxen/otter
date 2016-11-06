extern crate iron;
extern crate router;
extern crate staticfile;

use std::path::Path;

use iron::prelude::*;
use iron::status;

use router::Router;
use staticfile::Static;


fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World!")))
}


fn query_handler(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions
                       .get::<Router>()
                       .unwrap()
                       .find("query")
                       .unwrap_or("no query");
    let res = format!("Query is: {}", query);
    Ok(Response::with((status::Ok, res)))
}

fn main() {

    let mut router = Router::new();
    router.get("/", handler, "index");
    router.get("/:query", query_handler, "query");
    router.get("/static/",
               Static::new(Path::new("htdocs/index.html")),
               "static");
    let _server = Iron::new(router).http("localhost:8080").unwrap();
}
