extern crate iron;
extern crate router;
extern crate git2;
extern crate logger;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::fs;
use std::io;
use std::io::Read;

use iron::prelude::*;
use iron::status;

use router::Router;
use logger::Logger;

static REPO_PATH: &'static str = "pages/";
static STATIC_PATH: &'static str = "htdocs/";
static TEMPLATE_PATH: &'static str = "templates/";
static SERVER_ADDRESS: &'static str = "localhost:8080";

fn get_page(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions
                       .get::<Router>()
                       .unwrap()
                       .find("page")
                       .unwrap_or("no query");

    // let tmpl =
    let res = format!("Page is: {}", query);
    Ok(Response::with((status::Ok, res)))
}

fn post_page(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions
                       .get::<Router>()
                       .unwrap()
                       .find("page")
                       .unwrap_or("no query");
    match git2::Repository::open(REPO_PATH) {
        Ok(repo) => {}
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    let res = format!("Posted page: {}", query);
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
    match fs::File::open(staticpath) {
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
    env_logger::init().unwrap();
    info!("Starting...");
    let (logger_before, logger_after) = Logger::new(None);

    let mut router = Router::new();
    router.get("/:page", get_page, "page");
    router.post("/:page", post_page, "page");
    router.get("/static/:item", get_static, "static");


    let mut chain = Chain::new(router);

    chain.link_before(logger_before);
    chain.link_after(logger_after);
    let _server = Iron::new(chain).http(SERVER_ADDRESS).unwrap();
    info!("Server running on {}", SERVER_ADDRESS);
}
