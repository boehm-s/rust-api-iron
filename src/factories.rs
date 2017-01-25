extern crate iron;
extern crate iron_postgres_middleware as pg_middleware;
extern crate bodyparser;
extern crate pwhash;
extern crate r2d2_postgres;

use self::r2d2_postgres::{r2d2};

use utils::get_json_body;
use utils::verif_body;

use std::vec::Vec;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;

use pg_middleware::{PostgresReqExt};
//_____________________________________________________________________________________________________

pub fn create_type(req: &mut Request) -> IronResult<Response> {
    let res = get_json_body(req.get::<bodyparser::Json>());
    if !verif_body(vec!["name".to_string(), "image_url".to_string(), "product".to_string(), "level_multiplier".to_string(), "base_value".to_string(), "product_id".to_string()] , &res) {
        return Ok(Response::with((status::BadRequest)))
    }

    let con = req.db_conn();
    let obj = res.as_object().unwrap();

    let name = str::replace(&obj.get("name").unwrap().to_string(), "\"", "");
    let image_url = str::replace(&obj.get("image_url").unwrap().to_string(), "\"", "");
    let product = str::replace(&obj.get("product").unwrap().to_string(), "\"", "");
    let level_multiplier:f64 = obj.get("level_multiplier").unwrap().as_f64().unwrap();
    let base_value:i32 = obj.get("base_value").unwrap().as_f64().unwrap() as i32;
    let product_id:i32 = obj.get("product_id").unwrap().as_f64().unwrap() as i32;

    con.execute("INSERT INTO type_buildings(name, image_url, product, level_multiplier, base_value, product_id) VALUES($1, $2, $3, $4, $5, $6);", &[&name, &image_url, &product, &level_multiplier, &base_value, &product_id]).unwrap();

    Ok(Response::with((status::Ok)))
}

pub fn get_types(req: &mut Request) -> IronResult<Response> {
    let con = req.db_conn();
    let mut rows_string: Vec<String> = Vec::new();
    for row in &con.query("SELECT * FROM type_buildings", &[]).unwrap() {
        let bt0:i32 = row.get(0);
        let bt1:String = row.get(1);
        let bt2:String = row.get(2);
        let bt3:String = row.get(3);
        let bt4:f64 = row.get(4);
        let str_test = format!("{{\"id\":\"{}\", \"name\":\"{}\", \"image_url\":\"{}\", \"product\":\"{}\", \"level_multiplier\":\"{}\"}}", bt0, bt1, bt2, bt3, bt4);

        rows_string.push(str_test);
    }

    let mut out:String;
    if rows_string.len() > 0 {
        out = format!("[{}]", rows_string.iter().fold(String::new(), |a, b| format!("{},{}", a, b))[1..].to_string());
    } else {
        out = "[]".to_string();
    }
    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((status::Ok, content_type, out)))
}


pub fn get(req: &mut Request) -> IronResult<Response> {
    let user_id:i32 = req.url.to_string().split("/").last().unwrap().parse::<i32>().unwrap();

    let con = req.db_conn();

    let mut rows_string: Vec<String> = Vec::new();
    for row in &con.query("SELECT * FROM buildings WHERE user_id = $1;", &[&user_id]).unwrap() {
        let bt0:i32 = row.get(0);
        let bt1:i32 = row.get(1);
        let bt2:i32 = row.get(2);
        let bt3:i32 = row.get(3);
        let bt4:i32 = row.get(4);
        let str_test = format!("{{\"id\":\"{}\", \"level\":\"{}\", \"type\":\"{}\", \"user_id\":\"{}\", \"planet_id\":\"{}\"}}", bt0, bt1, bt2, bt3, bt4);

        rows_string.push(str_test);
    }


    let mut out:String = String::new();
    if rows_string.len() > 0 {
         out = format!("[{}]", rows_string.iter().fold(String::new(), |a, b| format!("{},{}", a, b))[1..].to_string());
    } else {
        out = "[]".to_string();
    }
    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((status::Ok, content_type, out)))
}


pub fn create(req: &mut Request) -> IronResult<Response> {
    let res = get_json_body(req.get::<bodyparser::Json>());
    if !verif_body(vec!["type".to_string(), "user_id".to_string(), "planet_id".to_string()] , &res) {
        return Ok(Response::with((status::BadRequest)))
    }

    let con = req.db_conn();
    let obj = res.as_object().unwrap();
    let building_type:i32 = obj.get("type").unwrap().as_i64().unwrap() as i32;
    let user_id:i32 = obj.get("user_id").unwrap().as_i64().unwrap() as i32;
    let planet_id:i32 = obj.get("planet_id").unwrap().as_i64().unwrap() as i32;

    let stmt = con.prepare("INSERT INTO buildings(level, type, user_id, planet_id) VALUES(1, $1, $2, $3) RETURNING id").unwrap();
    let id: i32 = stmt.query(&[&building_type, &user_id, &planet_id]).iter().next().unwrap().get(0).get(0);

    let str_id = format!("{{\"id\":\"{}\"}}", id);
    Ok(Response::with((status::Ok, str_id)))
}

pub fn level_up(req: &mut Request) -> IronResult<Response> {
    let res = get_json_body(req.get::<bodyparser::Json>());
    if !verif_body(vec!["building_id".to_string(), "user_id".to_string()] , &res) {
        return Ok(Response::with((status::BadRequest)))
    }

    let con = req.db_conn();
    let obj = res.as_object().unwrap();
    let building_id:i32 = obj.get("building_id").unwrap().as_i64().unwrap() as i32;
    let user_id:i32 = obj.get("building_id").unwrap().as_i64().unwrap() as i32;
    let ressources:UserStat = get_user_ressources(&con, user_id);
    let price:i32 = get_lvl_up_price(&con, building_id);

    if ressources.crystal < price {
        return Ok(Response::with((status::Unauthorized)));
    }

    con.execute("UPDATE user_ressources SET crystal = crystal - $1 WHERE user_id = $2;", &[&price, &user_id]).unwrap();
    let stmt = con.prepare("UPDATE buildings SET level = level + 1 WHERE id = $1 RETURNING level;").unwrap();
    let lvl:i32 = stmt.query(&[&building_id]).iter().next().unwrap().get(0).get(0);
    con.execute("INSERT INTO buildings_update_level(level, building_id) VALUES($1, $2);", &[&lvl, &building_id]).unwrap();

    Ok(Response::with((status::Ok)))
}


struct UserStat {
    energy: i32,
    crystal: i32,
    metal: i32
}

fn get_user_ressources(con: &r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>, user_id: i32) -> UserStat {
    let query = con.query("SELECT energy, crystal, metal FROM user_ressources WHERE user_id = $1;", &[&user_id]).unwrap();
    let res = query.get(0);

    UserStat {
        energy: res.get(0),
        crystal: res.get(1),
        metal: res.get(2),
    }
}

fn get_lvl_up_price(con: &r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>, building_id: i32) -> i32 {
    let mut price:i32 = 0;
    let query = &con.query("SELECT type_buildings.base_price FROM type_buildings, buildings WHERE buildings.type = type_buildings.id AND buildings.id = $1", &[&building_id]).unwrap();
    let base_price:i32 = query.get(0).get(0);
    let query2 = &con.query("SELECT level FROM buildings_update_level WHERE building_id = $1", &[&building_id]).unwrap();
    for row in query {
        let add:i32 = row.get(0);
        price = price + add * base_price;
    }
    price
}
