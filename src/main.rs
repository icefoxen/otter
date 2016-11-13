extern crate pencil;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hoedown;

use std::collections::BTreeMap;
use std::fs;
use std::io;

use pencil::{Pencil, Request, Response, PencilResult};
use pencil::http_errors;

use hoedown::Render;

static PAGE_PATH: &'static str = "pages/";

fn page_path(page: &str) -> String {
    let mut pagepath = PAGE_PATH.to_string();
    pagepath += page;
    pagepath += ".md";
    pagepath
}

fn page_get(request: &mut Request) -> PencilResult {
    let page = request.view_args.get("page").unwrap();
    let pagepath = page_path(page);
    match fs::File::open(pagepath) {
        Ok(file) => {
            let md = hoedown::Markdown::read_from(file);
            let mut html = hoedown::Html::new(hoedown::renderer::html::Flags::empty(), 0);
            let buffer = html.render(&md);
            let rendered_markdown = buffer.to_str().unwrap();

            Ok(Response::from(rendered_markdown))
        }
        Err(e) => {
            let status = match e.kind() {
                io::ErrorKind::NotFound => http_errors::NotFound,
                io::ErrorKind::PermissionDenied => http_errors::Forbidden,
                _ => http_errors::InternalServerError,
            };

            return pencil::abort(status.code())
        }
    }
}

fn page_post(_: &mut Request) -> PencilResult {
    Ok(Response::from("Posted page!"))
}

fn test_template_get(request: &mut Request) -> PencilResult {
    let mut ctx = BTreeMap::new();
    ctx.insert("name".to_string(), "template".to_string());
    return request.app.render_template("hello.html", &ctx);
}

fn setup_app() -> Pencil {
    let mut app = Pencil::new(".");
    app.set_debug(true);
    app.enable_static_file_handling();
    app.register_template("hello.html");
    app.get("/hello_template", "hello_template", test_template_get);
    app.get("/<page:string>", "page_get", page_get);
    app.post("/<page:string>", "page_post", page_post);
    app
}

fn main() {
    let app = setup_app();
    debug!("* Running on http://localhost:5000/");
    app.run("localhost:5000");
}

mod test {

    // Well it turns out it's a pain in the butt to actually
    // create unit tests, because it's a pain in the butt to
    // actually create a pencil Request object without a 
    // network connection involved.  See Pencil issue #41.
    // So, actually starting the server here might well be the
    // best way to run unit tests on it.
    

    #[test]
    fn it_works() {
        let req = ();
        //page_get(request)
    }
}