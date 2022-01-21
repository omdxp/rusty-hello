use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("hello.txt").expect("Could not create file!");

    file.write_all(b"Hello, fish!")
        .expect("Could not write to file!");
}
