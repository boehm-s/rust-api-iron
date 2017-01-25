extern crate iron;
extern crate iron_postgres_middleware as pg_middleware;
extern crate bodyparser;
extern crate serde_json;
extern crate pwhash;

use self::pwhash::bcrypt;

use utils::get_json_body;
use utils::verif_body;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;

use pg_middleware::{PostgresReqExt};
//_____________________________________________________________________________________________________
const START_METAL:i32 = 200;
const START_ENERGY:i32 = 100;
const START_CRYSTAL:i32 = 50;

pub fn register(req: &mut Request) -> IronResult<Response> {
    let res = get_json_body(req.get::<bodyparser::Json>());
    if !verif_body(vec!["email".to_string(), "username".to_string(), "password".to_string()] , &res) {
        return Ok(Response::with((status::BadRequest)))
    }

    let con = req.db_conn();
    let obj = res.as_object().unwrap();

    let email = str::replace(&obj.get("email").unwrap().to_string(), "\"", "");
    let username = str::replace(&obj.get("username").unwrap().to_string(), "\"", "");
    let password = str::replace(&obj.get("password").unwrap().to_string(), "\"", "");
    let hash = bcrypt::hash(&password.to_string()).unwrap();

    // get user_id

    let stmt = con.prepare("INSERT INTO users (email, username, password) VALUES ($1, $2, $3) RETURNING id;").unwrap();
    let id: i32 = stmt.query(&[&email, &username, &hash]).iter().next().unwrap().get(0).get(0);
    con.execute("INSERT INTO user_ressources (user_id, energy, crystal, metal) VALUES ($1, $2, $3, $4);", &[&id, &START_ENERGY, &START_CRYSTAL, &START_METAL]).unwrap();


    let out = format!("{{\"id\": \"{}\"}}", id);
    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((status::Ok, content_type, out)))
}

pub fn auth(req: &mut Request) -> IronResult<Response> {
    let res = get_json_body(req.get::<bodyparser::Json>());
    if !verif_body(vec!["email".to_string(), "username".to_string()] , &res) {
        return Ok(Response::with((status::BadRequest)))
    }

    let con = req.db_conn();
    let obj = res.as_object().unwrap();

    let email = str::replace(&obj.get("email").unwrap().to_string(), "\"", "");
    let password = str::replace(&obj.get("password").unwrap().to_string(), "\"", "");
    let user_hash_req = &con.query("SELECT password, id FROM users WHERE email = $1", &[&email]).unwrap();

    let user_hash:String = user_hash_req.get(0).get(0);
    let id:i32 = user_hash_req.get(0).get(1);
    let out = format!("{{\"id\": \"{}\"}}", id);

    let content_type = "application/json".parse::<Mime>().unwrap();

    if bcrypt::verify(&password, &user_hash) == true {
        Ok(Response::with((status::Ok, content_type, out)))
    } else {
        Ok(Response::with((status::Unauthorized)))
    }
}
