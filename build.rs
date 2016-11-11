extern crate schemafy;

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
    let schema = "src/schema.json";
    println!("cargo:rerun-if-changed={}", schema);
    let src = Path::new(schema);

    let mut file = File::open(src).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let output = schemafy::generate(None, &input).unwrap();
    let dst = Path::new("src/lib.rs");

    let mut file = File::create(dst).unwrap();
    file.write_all(br#"
#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

"#)
        .unwrap();
    file.write_all(output.as_bytes()).unwrap();
}
