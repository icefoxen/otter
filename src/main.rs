extern crate iron;
extern crate router;
extern crate git2;
extern crate logger;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hoedown;
extern crate handlebars_iron;
extern crate rustc_serialize;

use std::error::Error;
use std::fs;
use std::io;

use iron::prelude::*;
use iron::status;

use router::Router;
use logger::Logger;

use rustc_serialize::json::ToJson;

use hoedown::Render;

static REPO_PATH: &'static str = "pages/";
static STATIC_PATH: &'static str = "htdocs/";
static TEMPLATE_PATH: &'static str = "templates/";
static SERVER_ADDRESS: &'static str = "localhost:8080";


#[derive(RustcDecodable, RustcEncodable)]
struct PageInfo {
    body: String,
}

impl PageInfo {
    fn new(body: &str) -> PageInfo {
        PageInfo { body: body.to_owned() }
    }
}

// impl ToJson for PageInfo {
//     fn to_json(&self) -> rustc_serialize::json::Json {}
// }

fn get_page(req: &mut Request) -> IronResult<Response> {
    let ref pagename = req.extensions
                          .get::<Router>()
                          .unwrap()
                          .find("page")
                          .unwrap_or("no query");

    let mut pagepath = REPO_PATH.to_owned();
    pagepath += pagename;
    pagepath += ".md";

    match fs::File::open(pagepath) {
        Ok(file) => {
            let md = hoedown::Markdown::read_from(file);

            let mut html = hoedown::Html::new(hoedown::renderer::html::Flags::empty(), 0);
            let buffer = html.render(&md);
            // This is bugged!
            // See https://github.com/iron/iron/issues/498
            // let br: BodyReader<hoedown::Buffer> = BodyReader(buffer);
            // Ok(Response::with((status::Ok, br)))
            // TODO: Set content-type
            let stringggggg = buffer.to_str().unwrap();
            let pageinfo = PageInfo::new(stringggggg);
            let json = rustc_serialize::json::encode(&pageinfo).unwrap();
            println!("JSON is: {}", json);
            // Ok(Response::with((status::Ok, stringggggg)));

            let t = handlebars_iron::Template::new("page", json);

            Ok(Response::with((status::Ok, t)))
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

    // HandlebarsEngine will look up all files with "./examples/templates/**/*.hbs"
    let mut hbse = handlebars_iron::HandlebarsEngine::new();
    hbse.add(Box::new(handlebars_iron::DirectorySource::new(TEMPLATE_PATH, ".tmpl")));

    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        panic!("{}", r.description());
    }


    let mut chain = Chain::new(router);

    chain.link_before(logger_before);

    chain.link_after(hbse);
    chain.link_after(logger_after);
    let _server = Iron::new(chain).http(SERVER_ADDRESS).unwrap();
    info!("Server running on {}", SERVER_ADDRESS);
}
