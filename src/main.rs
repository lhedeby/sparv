use std::{env, fs};

use crate::{interpreter::Interpreter, parser::Parser, scanner::Scanner};

mod scanner;
mod parser;
mod interpreter;

fn main() {
    println!("Gran compiler started...");
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => run_file(args.last().unwrap()),
        _ => panic!("Unacceptable usage"),
    }
    println!("Gran compiler stopped...");
}

fn run_file(file_path: &str) {
    match fs::read_to_string(file_path) {
        Ok(source) => {
            println!("Read file:");
            println!("{source}");
            let tokens = Scanner::get_tokens(source);
            let tree = match Parser::parse(tokens) {
                Ok(res) => res,
                Err(e) => panic!("Error parsing {:?}", e)
            };
            println!("||| Starting program |||");
            Interpreter::interpret(tree);

        },
        Err(e) => panic!("Error reading file {e}"),
    }
}
