extern crate bodyparser;
extern crate serde_json;

use std::vec::Vec;
use self::serde_json::Value;
//_____________________________________________________________________________________________________


pub fn get_json_body(json_body: Result<Option<Value>, bodyparser::BodyError>) -> Value {
    let mut res: Value = serde_json::from_str("{}").unwrap();

    match json_body {
        Ok(Some(json_body)) => res = json_body,
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err)
    }
    res
}

pub fn verif_body(values: Vec<String>, body_value: &Value) -> bool {
    let body_obj = body_value.as_object().unwrap();
    let count: i32 = values.iter().map(|value| {
        if body_obj.get(value).is_some() == true {
            0
        } else {
            1
        }
    }).fold(0, |acc, x| acc + x);

    if count > 0 {
        false
    } else {
        true
    }
}
