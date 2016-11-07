extern crate iron;
extern crate router;

use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;

use iron::prelude::*;
use iron::status;

use router::Router;

static REPO_PATH: &'static str = "pages/";
static STATIC_PATH: &'static str = "htdocs/";
static SERVER_ADDRESS: &'static str = "localhost:8080";

fn get_page(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions
                       .get::<Router>()
                       .unwrap()
                       .find("page")
                       .unwrap_or("no query");
    let res = format!("Page is: {}", query);
    Ok(Response::with((status::Ok, res)))
}


fn get_static(req: &mut Request) -> IronResult<Response> {
    let query = req.extensions
                   .get::<Router>()
                   .unwrap()
                   .find("item")
                   .unwrap_or("dne");
    let mut staticpath = STATIC_PATH.to_owned();
    staticpath += query;
    let path = Path::new(&staticpath);
    match fs::File::open(path) {
        Ok(file) => {
            // TODO: Detect content-type
            Ok(Response::with((status::Ok, file)))
        }
        Err(e) => {
            let status = match e.kind() {
                io::ErrorKind::NotFound => status::NotFound,
                io::ErrorKind::PermissionDenied => status::Forbidden,
                _ => status::InternalServerError,
            };

            Err(IronError::new(e, status))
        }
    }
}

fn main() {
    let mut router = Router::new();
    router.get("/:page", get_page, "page");
    router.get("/static/:item", get_static, "static");
    let _server = Iron::new(router).http(SERVER_ADDRESS).unwrap();
    println!("Server running on {}", SERVER_ADDRESS);
}
