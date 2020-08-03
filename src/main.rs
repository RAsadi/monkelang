use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod errors;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("interpreter not implemented");
    } else {
        let file_name = &args[1];
        let file = File::open(file_name).expect("Failed to read file");
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader
            .read_to_string(&mut contents)
            .expect("Failed to read file");
        parser::Parser::new(&contents.to_string()).expect("Failed to parse file");
    }
}
