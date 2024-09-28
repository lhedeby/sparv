use std::{env, fmt::Display, fs};
// use pub fn instead of impl
use crate::{interpreter::Interpreter, parser::Parser, scanner::Scanner};

mod interpreter;
mod parser;
mod scanner;
mod token;

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    println!("Gran compiler started...");
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => run_file(args.last().unwrap()),
        _ => panic!("Expected file path as only parameter"),
    }
    println!("Gran compiler stopped...");
}

pub fn run_file(file_path: &str) {
    match fs::read_to_string(file_path) {
        Ok(source) => {
            let tokens = match Scanner::get_tokens(source) {
                Ok(res) => res,
                Err(e) => {
                    println!("{} at {}:", file_path, e.line);
                    println!("Lexing Error - {}", e);
                    return;
                }
            };
            let tree = match Parser::parse(tokens) {
                Ok(res) => res,
                Err(e) => {
                    println!("{} at {}:", file_path, e.line);
                    println!("Parsing Error - {}", e);
                    return;
                }
            };
            match Interpreter::interpret(tree) {
                Ok(_) => println!("successfully ran program"),
                Err(e) => {
                    println!("Runtime Error - {:?}", e);
                }
            }
        }
        Err(e) => println!("Error reading file {e}"),
    }
}

pub struct Error {
    line: usize,
    kind: ErrorKind,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::UnexpectedCharacter => write!(f, "Unexpected character"),
            ErrorKind::UnterminatedString => write!(f, "Unterminated string"),
            ErrorKind::Unknown => write!(f, "An unknown error has occured"),
            ErrorKind::UnexpectedToken => write!(f, "Unexpected token"),
            ErrorKind::Assignment => write!(f, "Invalid assignment"),
            ErrorKind::Import(file_name) => write!(f, "Could not import file: '{file_name}'")
        }
    }
}

pub enum ErrorKind {
    UnexpectedCharacter,
    UnterminatedString,
    UnexpectedToken,
    Assignment,
    Import(String),
    Unknown,
}
