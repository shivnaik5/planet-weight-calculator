use std::io;
use std::env;
use serde_json::{Value, Error};

mod cli;
mod json;

fn main() {
    let args = cli::get_args();
    println!("{} {}", args.weight, args.planet);

    let planets: Value = json::read_json();
    println!("{}", planets);
}
