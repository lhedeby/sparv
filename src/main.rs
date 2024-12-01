use crate::{error::print_error, token::TokenKind};
use std::{env, fs};

use crate::{interpreter::Interpreter, parser::Parser, scanner::Scanner};

mod error;
mod interpreter;
mod parser;
mod scanner;
mod token;
mod lsp;
mod analysis;
mod native_function;

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "lsp") {
        lsp::server::start();
    }
    match args.len() {
        2 => run_file(args.last().unwrap()),
        _ => panic!("Expected file path as only parameter"),
    }
}

pub fn run_file(file_path: &str) {
    if file_path.split('.').last().unwrap() != "sparv" {
        println!("'{}' is not a .sparv file", file_path);
        std::process::exit(1)
    }
    match fs::read_to_string(file_path) {
        Ok(source) => {

            println!("Running: {file_path}");
            let tree = match Parser::parse(&source, file_path) {
                Ok(res) => res,
                Err(e) => {
                    println!("Parsing Error");
                    print_error(e, file_path, &source);
                    return;
                }
            };
            match Interpreter::interpret(tree) {
                Ok(_) => println!("successfully ran program"),
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
        Err(e) => println!("Error reading file: {e}"),
    }
}
