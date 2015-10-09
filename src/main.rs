extern crate markdown;

use std::fs::File;

use std::io::{Read, Write};

fn main() {
    let mut md = File::open("test.md").unwrap();
    let mut html = File::create("test.html").unwrap();
    let mut s = String::new();
    md.read_to_string(&mut s).unwrap();
    html.write_all(&markdown::to_html(&s).into_bytes()).unwrap();
}
