extern crate core;

use std::fmt;
use std::fmt::{Display, Formatter};
use serde_json::{Error, Value};

fn get_name_from_json_string(json: &str) -> String {
    let parsed: Result<Value, Error> = serde_json::from_str(json);

    match parsed {
        Ok(r) => {
            match &r["name"] {
                Value::String(n) => n.clone(),
                _ => panic!("no name field")
            }
        },
        Err(_) => panic!("could not parse")
    }
}

#[derive(Debug)]
struct NameError {
    msg: String
}

impl NameError {
    fn of(msg: &str) -> NameError {
        return NameError { msg: msg.to_string() };
    }
}

impl Display for NameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.msg.as_str())
    }
}

fn safe_get_name_from_json_string(json: &str) -> Result<String, NameError> {
    let parsed: Result<Value, Error> = serde_json::from_str(json);

    match parsed {
        Ok(r) => {
            match &r["name"] {
                Value::String(n) => Ok(n.clone()),
                _ => Err(NameError::of("missing name field"))
            }
        },
        Err(_) => Err(NameError::of("failed to parse"))
    }
}

fn maybe_get_name_from_json_string(json: &str) -> Option<String> {
    let parsed: Result<Value, Error> = serde_json::from_str(json);

    match parsed {
        Ok(r) => {
            match &r["name"] {
                Value::String(n) => Option::Some(n.clone()),
                _ => None
            }
        },
        Err(_) => None
    }
}

fn main() {
    let name = get_name_from_json_string(r#"{
        "name": "dionysus"
    }"#);

    println!("name was - {}", &name[..]);

    let name_err = safe_get_name_from_json_string(r#"{
        "age": 904
    }"#).unwrap_err();

    println!("name error was - {}", name_err.msg);

    let parse_err = safe_get_name_from_json_string(r#"}"#).unwrap_err();

    println!("parse error was - {}", parse_err.msg);

    let option_name = maybe_get_name_from_json_string(r#"{
        "name": "dionysus"
    }"#);

    println!("name was - {}", &option_name.unwrap()[..]);

    let missing_name = maybe_get_name_from_json_string(r#"{
        "age": 904
    }"#);

    println!("is the name missing? {}", missing_name == None);
}
