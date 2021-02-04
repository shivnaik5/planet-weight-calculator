use std::fs::File;
use std::io::Read;
// use serde_json::value::Value;
use serde_json::Result;
use serde_json::{Value, Error};

pub fn read_json() -> Result<Value, Error> {
    let mut file = File::open("src/planet_conversions.json").unwrap();
    let data: Value = serde_json::from_reader(file)
        .expect("File should be proper JSON");

    Ok(data);
}