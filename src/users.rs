extern crate iron;
extern crate bodyparser;
extern crate serde_json;
extern crate pwhash;

use self::pwhash::bcrypt;

use utils::get_json_body;
use utils::verif_body;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;

//_____________________________________________________________________________________________________

pub fn register(req: &mut Request) -> IronResult<Response> {
    let res = get_json_body(req.get::<bodyparser::Json>());
    if !verif_body(vec!["email".to_string(), "username".to_string(), "password".to_string()] , &res) {
        return Ok(Response::with((status::BadRequest)))
    }

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((status::Ok, content_type)))
}

pub fn auth(req: &mut Request) -> IronResult<Response> {
    let res = get_json_body(req.get::<bodyparser::Json>());
    if !verif_body(vec!["email".to_string(), "username".to_string()] , &res) {
        return Ok(Response::with((status::BadRequest)))
    }

    let password = String::from("password");
    let user_hash = String::from("hey i'm a hash !");
    let content_type = "application/json".parse::<Mime>().unwrap();

    if bcrypt::verify(&password, &user_hash) == true {
        Ok(Response::with((status::Ok, content_type)))
    } else {
        Ok(Response::with((status::Unauthorized)))
    }
}
