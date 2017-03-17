extern crate iron;
extern crate mount;
extern crate router;

use mount::Mount;
use router::Router;
use api;

let mut mount = Mount::new();
let mut router = Router::new();

router
    .get("/health_test", api::controllers::health_test, "health_test");

mount
    .mount("/", router)
    .mount("/users", api::routes::users)
