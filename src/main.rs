use std::fs::File;
use std::io::Read;

pub mod lexer;
pub mod parser;

fn main() {
    let file_path = "";
    let mut f =
        File::open(file_path).unwrap_or_else(|e| panic!("Invalid file receirved error: {:?}", e));

    let mut file = String::new();
    f.read_to_string(&mut file);
    println!("Hello, world!");
}
