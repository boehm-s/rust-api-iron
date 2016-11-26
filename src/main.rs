extern crate iron;
extern crate staticfile;
extern crate router;
extern crate mount;

use mount::Mount;
use router::Router;
use staticfile::Static;

use iron::prelude::*;
use iron::status;

use std::path::Path;

fn health_test(req: &mut Request) -> IronResult<Response> {
    println!("Running health_test handler, URL path: {}", req.url.path().join("/"));
    Ok(Response::with((status::Ok, "This request was routed (health_test succeded) !")))
}

fn main() {
    let mut router = Router::new();
    router
        .get("/health_health_test", health_test, "health_test");

    let mut mount = Mount::new();
    mount
        .mount("/", router)
        .mount("/html", Static::new(Path::new("public/index.html")))
        .mount("/js/", Static::new(Path::new("public/js")));

    Iron::new(mount).http("127.0.0.1:3000").unwrap();
}
