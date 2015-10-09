extern crate markdown;

use std::fs;
use std::fs::File;

use std::io::{Read, Write};

fn main() {
    fs::create_dir_all("html").unwrap();
    let mut md = File::open("test.md").unwrap();
    let mut html = File::create("html/test.html").unwrap();
    let mut s = String::new();
    md.read_to_string(&mut s).unwrap();
    html.write_all(&markdown::to_html(&s).into_bytes()).unwrap();
}
