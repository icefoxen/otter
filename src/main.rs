extern crate pencil;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hoedown;

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::io::Read;

use pencil::helpers;
use pencil::{Pencil, Request, Response, PencilResult, PencilError};
use pencil::http_errors;

use hoedown::Render;

static PAGE_PATH: &'static str = "pages/";

fn page_path(page: &str) -> String {
    let mut pagepath = PAGE_PATH.to_string();
    pagepath += page;
    pagepath += ".md";
    pagepath
}

fn load_page_file(pagename: &str) -> Result<String, PencilError> {
    let pagepath = page_path(pagename);
    match fs::File::open(pagepath) {
        Ok(mut file) => {
            let mut s = String::new();
            let _ = file.read_to_string(&mut s).unwrap();
            Ok(s)
        }
        Err(e) => {
            let status = match e.kind() {
                io::ErrorKind::NotFound => http_errors::NotFound,
                io::ErrorKind::PermissionDenied => http_errors::Forbidden,
                _ => http_errors::InternalServerError,
            };

            let err = PencilError::PenHTTPError(status);
            return Err(err)
        }
    }
}

fn index_redirect(_request: &mut Request) -> PencilResult {
    // Permanent redirect, cache-able.
    helpers::redirect("/index", 308)
}

fn page_get(request: &mut Request) -> PencilResult {
    let page = request.view_args.get("page").unwrap();
    let contents = load_page_file(page)?;

    let md = hoedown::Markdown::from(contents.as_bytes());
    let mut html = hoedown::Html::new(hoedown::renderer::html::Flags::empty(), 0);
    let buffer = html.render(&md);
    let rendered_markdown = buffer.to_str().unwrap();

    // VARIABLES THAT NEED TO EXIST:
    // root path
    // Header
    // footer
    
    let mut ctx = BTreeMap::new();
    ctx.insert("pagename".to_string(), page.to_string());
    ctx.insert("page".to_string(), rendered_markdown.to_string());
    
    request.app.render_template("page.html", &ctx)
}

fn page_edit_get(request: &mut Request) -> PencilResult {
    let page = request.view_args.get("page").unwrap();
    let contents = load_page_file(page)?;

    let mut ctx = BTreeMap::new();
    ctx.insert("title".to_string(), page.to_string());
    ctx.insert("page".to_string(), contents.to_string());
    
    request.app.render_template("edit.html", &ctx)
}
fn page_edit_post(request: &mut Request) -> PencilResult {
    println!("Edit posted thing");
    let newpage = request.form().get("submission").unwrap();
    let response = format!("Posted editing page: {}", newpage);
    Ok(Response::from(response))
}

fn setup_app() -> Pencil {
    let mut app = Pencil::new(".");
    app.set_debug(true);
    app.enable_static_file_handling();
    app.register_template("page.html");
    app.register_template("edit.html");
    app.get("/", "index", index_redirect);
    app.get("/<page:string>", "page_get", page_get);
    app.get("/edit/<page:string>", "page_edit_get", page_edit_get);
    app.post("/edit/<page:string>", "page_edit_post", page_edit_post);
    app
}

static ADDRESS: &'static str = "localhost:5000";

fn main() {
    let app = setup_app();
    app.run(ADDRESS);
}

mod test {

    //use std::process::{Command, Child};

    // Well it turns out it's a pain in the butt to actually
    // create unit tests, because it's a pain in the butt to
    // actually create a pencil Request object without a 
    // network connection involved.  See Pencil issue #41.
    // So, actually starting the server here might well be the
    // best way to run unit tests on it.
    /*
    fn start_test_server() -> Child {
        let child = Command::new("cargo")
            .arg("run")
            .spawn()
            .unwrap();
        child
    }

    fn curl(url: &str) -> Child {
        let child = Command::new("curl")
            .arg(url)
            .spawn()
            .unwrap();
        child
    }
    */
/*
    #[test]
    fn it_works() {
        let mut c = start_test_server();
        //c.wait().unwrap();
        let mut curl = curl("http://localhost:5000/start");
        curl.wait().unwrap();
        // Goodness, no TERM signal?  How violent.
        c.kill().unwrap();
    }
*/
}
