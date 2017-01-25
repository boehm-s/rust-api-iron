extern crate iron;
extern crate iron_postgres_middleware as pg_middleware;
extern crate bodyparser;
extern crate pwhash;
extern crate r2d2_postgres;

use self::r2d2_postgres::{r2d2};

use self::pwhash::bcrypt;

use utils::get_json_body;
use utils::verif_body;

use std::io::Read;
use std::vec::Vec;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;

use pg_middleware::{PostgresReqExt};
//_____________________________________________________________________________________________________

const LVL_UP_ENERGY_COST:i32 = 5;
const LVL_UP_MULTIPLIER:i32 = 2;
// lvl 7 costs 5 * 2^7
const LOOSE_DROID_LVL_DIFF:i32 = 5;
const ESCAPE_LVL_MULTIPLICATOR:i32 = 2;
// lvl5 attacks lvl10 --> lvl5 give rand(0, 5*5 - escape_lvl * ESCAPE_LVL_MULTIPLICATOR) droid to lvl10
const CHANCE_GET_DROID:i32 = 10;
// 10% of the entire ennemy fleet

pub fn create(req: &mut Request) -> IronResult<Response> {
    let user_id:i32 = req.url.to_string().split("/").last().unwrap().parse::<i32>().unwrap();

    let con = req.db_conn();
    con.execute("INSERT INTO droids(user_id, attack_level, defense_level, escape_level) VALUES($1, 1, 1, 1);", &[&user_id]).unwrap();

    Ok(Response::with((status::Ok)))
}

pub fn get(req: &mut Request) -> IronResult<Response> {
    let user_id:i32 = req.url.to_string().split("/").last().unwrap().parse::<i32>().unwrap();

    let con = req.db_conn();

    let mut rows_string: Vec<String> = Vec::new();
    for row in &con.query("SELECT * FROM droids WHERE user_id = $1;", &[&user_id]).unwrap() {
        let bt0:i32 = row.get(0);
        let bt1:i32 = row.get(1);
        let bt2:i32 = row.get(2);
        let bt3:i32 = row.get(3);
        let bt4:i32 = row.get(4);
        let str_test = format!("{{\"id\":\"{}\", \"user_id\":\"{}\", \"attack_level\":\"{}\", \"defense_level\":\"{}\", \"escape_level\":\"{}\"}}", bt0, bt1, bt2, bt3, bt4);
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

pub fn level_up_attack(req: &mut Request) -> IronResult<Response> {
    let res = get_json_body(req.get::<bodyparser::Json>());
    if !verif_body(vec!["user_id".to_string(), "droid_id".to_string()] , &res) {
        return Ok(Response::with((status::BadRequest)))
    }

    let con = req.db_conn();
    let obj = res.as_object().unwrap();
    let user_id:i32 = obj.get("user_id").unwrap().as_f64().unwrap() as i32;
    let droid_id:i32 = obj.get("droid_id").unwrap().as_f64().unwrap() as i32;

    let ressources: UserDroidStat = get_user_ressources(&con, user_id, droid_id);
    let metal_cost: i32 = LVL_UP_MULTIPLIER * LVL_UP_ENERGY_COST.pow(ressources.attack_level as u32);

    if ressources.metal < metal_cost {
        return Ok(Response::with((status::Unauthorized)))
    }

    con.execute("UPDATE droids SET attack_level = attack_level + 1 WHERE id = $1;", &[&droid_id]).unwrap();
    con.execute("UPDATE user_ressources SET metal = metal - $1 WHERE user_id = $2;", &[&metal_cost, &user_id]).unwrap();
    Ok(Response::with((status::Ok)))
}


pub fn level_up_defense(req: &mut Request) -> IronResult<Response> {
    let res = get_json_body(req.get::<bodyparser::Json>());
    if !verif_body(vec!["user_id".to_string(), "droid_id".to_string()] , &res) {
        return Ok(Response::with((status::BadRequest)))
    }

    let con = req.db_conn();
    let obj = res.as_object().unwrap();
    let user_id:i32 = obj.get("user_id").unwrap().as_f64().unwrap() as i32;
    let droid_id:i32 = obj.get("droid_id").unwrap().as_f64().unwrap() as i32;

    let ressources: UserDroidStat = get_user_ressources(&con, user_id, droid_id);
    let metal_cost: i32 = LVL_UP_ENERGY_COST * LVL_UP_MULTIPLIER.pow(ressources.defense_level as u32);
    if ressources.metal < metal_cost {
        return Ok(Response::with((status::Unauthorized)))
    }

    con.execute("UPDATE droids SET defense_level = defense_level + 1 WHERE id = $1;", &[&droid_id]).unwrap();
    con.execute("UPDATE user_ressources SET metal = metal - $1 WHERE user_id = $2;", &[&metal_cost, &user_id]).unwrap();
    Ok(Response::with((status::Ok)))
}


pub fn level_up_escape(req: &mut Request) -> IronResult<Response> {
    let res = get_json_body(req.get::<bodyparser::Json>());
    if !verif_body(vec!["user_id".to_string(), "droid_id".to_string()] , &res) {
        return Ok(Response::with((status::BadRequest)))
    }

    let con = req.db_conn();
    let obj = res.as_object().unwrap();
    let user_id:i32 = obj.get("user_id").unwrap().as_f64().unwrap() as i32;
    let droid_id:i32 = obj.get("droid_id").unwrap().as_f64().unwrap() as i32;

    let ressources: UserDroidStat = get_user_ressources(&con, user_id, droid_id);
    let metal_cost: i32 = LVL_UP_ENERGY_COST * LVL_UP_MULTIPLIER.pow(ressources.escape_level as u32);
    if ressources.metal < metal_cost {
        return Ok(Response::with((status::Unauthorized)))
    }

    con.execute("UPDATE droids SET escape_level = escape_level + 1 WHERE id = $1;", &[&droid_id]).unwrap();
    con.execute("UPDATE user_ressources SET metal = metal - $1 WHERE user_id = $2;", &[&metal_cost, &user_id]).unwrap();
    Ok(Response::with((status::Ok)))
}



struct UserDroidStat {
    attack_level: i32,
    defense_level: i32,
    escape_level: i32,
    energy: i32,
    crystal: i32,
    metal: i32
}

fn get_user_ressources(con: &r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>, user_id: i32, droid_id: i32) -> UserDroidStat {
    let query = con.query("SELECT user_ressources.energy, user_ressources.crystal, user_ressources.metal, droids.attack_level, droids.defense_level, droids.escape_level FROM user_ressources, droids WHERE user_ressources.user_id = $1 AND droids.id = $2;", &[&user_id, &droid_id]).unwrap();
    let res = query.get(0);

    UserDroidStat {
        energy: res.get(0),
        crystal: res.get(1),
        metal: res.get(2),
        attack_level: res.get(3),
        defense_level: res.get(4),
        escape_level: res.get(5),
    }
}
