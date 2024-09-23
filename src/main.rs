use std::{env, fs};
// use pub fn instead of impl
use crate::{interpreter::Interpreter, parser::Parser, scanner::Scanner};

mod interpreter;
mod parser;
mod scanner;
mod token;

fn main() {
    println!("Gran compiler started...");
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => run_file(args.last().unwrap()),
        _ => panic!("Expected file path as only parameter"),
    }
    println!("Gran compiler stopped...");
}

fn run_file(file_path: &str) {
    match fs::read_to_string(file_path) {
        Ok(source) => {
            println!("Read file:");
            println!("{source}");
            let tokens = match Scanner::get_tokens(source) {
                Ok(res) => res,
                Err(e) => panic!("Error scanning {:?}", e),
            };
            let tree = match Parser::parse(tokens) {
                Ok(res) => res,
                Err(e) => panic!("Error parsing {:?}", e),
            };
            println!("||| Starting program |||");
            match Interpreter::interpret(tree) {
                Ok(_) => println!("successfully ran program"),
                Err(e) => panic!("Err {:?}", e),
            }
        }
        Err(e) => panic!("Error reading file {e}"),
    }
}

