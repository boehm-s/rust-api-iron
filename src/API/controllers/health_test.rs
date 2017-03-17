extern crate iron;

use iron::prelude::*;
use iron::status;

pub fn health_test(req: &mut Request) -> IronResult<Response> {
    println!("Running health_test handler, URL path: {}", req.url.path().join("/"));
    Ok(Response::with((status::Ok, "This request was routed (health_test succeded) !")))
}
