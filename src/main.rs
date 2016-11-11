extern crate pencil;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::collections::BTreeMap;

use pencil::{Pencil, Request, Response, PencilResult};

fn page_get(_: &mut Request) -> PencilResult {
    Ok(Response::from("Hello World!"))
}

fn page_post(_: &mut Request) -> PencilResult {
    Ok(Response::from("Posted page!"))
}

fn test_template_get(request: &mut Request) -> PencilResult {
    let mut ctx = BTreeMap::new();
    ctx.insert("name".to_string(), "template".to_string());
    return request.app.render_template("hello.html", &ctx);
}
fn main() {
    let mut app = Pencil::new(".");
    app.set_debug(true);
    app.enable_static_file_handling();
    app.register_template("hello.html");
    app.get("/hello_template", "hello_template", test_template_get);
    app.get("/<page:string>", "page_get", page_get);
    app.post("/<page:string>", "page_post", page_post);
    debug!("* Running on http://localhost:5000/");
    app.run("localhost:5000");
}
