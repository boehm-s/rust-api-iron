extern crate iron;
extern crate staticfile;
extern crate router;
extern crate mount;
extern crate ws;

mod users;
mod utils;

use mount::Mount;
use router::Router;
use staticfile::Static;

use std::thread;
use std::path::Path;

use iron::prelude::*;
use iron::status;

use ws::listen;

//_____________________________________________________________________________________________________

fn health_test(req: &mut Request) -> IronResult<Response> {
    println!("Running health_test handler, URL path: {}", req.url.path().join("/"));
    Ok(Response::with((status::Ok, "This request was routed (health_test succeded) !")))
}


fn main() {
    let mut player_router = Router::new();
    player_router
        .get("/health_test", health_test, "health_test")
        .post("/register", users::register, "register")
        .post("/login", users::auth, "auth");


    let mut mount = Mount::new();
    mount
        .mount("/api/1/players/", player_router);
        // .mount("/api/1/factories/", factory_router)
        // .mount("/api/1/droids/", droid_router)
        // .mount("/html", Static::new(Path::new("public/index.html")))
        // .mount("/img/", Static::new(Path::new("public/img")))
        // .mount("/css/", Static::new(Path::new("public/css")))
        // .mount("/font/", Static::new(Path::new("public/font")))
        // .mount("/js/", Static::new(Path::new("public/js")));


    thread::spawn( move || {
        if let Err(error) = listen("127.0.0.1:3001", |out| {
            move |msg| {
                println!("Server got message '{}'. ", msg);
                out.send(msg)
            }
        }) {
            println!("Failed to create WebSocket due to {:?}", error);
        }
    });


    let mut chain = Chain::new(mount);
    Iron::new(chain).http("127.0.0.1:3000").unwrap();
}
