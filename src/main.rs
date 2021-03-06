use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod errors;
mod interpreter;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("REPL not implemented");
    } else {
        let file_name = &args[1];
        let file = File::open(file_name).expect("Failed to read file");
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).expect("Failed to read file");
        let parser = parser::Parser::tokenize(&contents.to_string()).expect("Failed to parse file");

        let mut interpreter = interpreter::Interpreter::new();
        interpreter.interpret(parser.get_tokens())
    }
}
