use std::fs::File;
use serde_json::{Result, Value, Error}; 

pub fn read_json(file_name: String) -> Result<Value> {
    let file = File::open(file_name).unwrap();
    let data: Value = serde_json::from_reader(file)
        .expect("File should be proper JSON!");

    Ok(data)
}