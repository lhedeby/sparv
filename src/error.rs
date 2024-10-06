use std::fmt::Display;

use crate::token::TokenKind;

pub type Result<T> = std::result::Result<T, Error>;

pub fn print_error(e: Error, file_path: &str, source: &str) {
    let mut lines = source.lines().collect::<Vec<&str>>();
    lines.push("<End of file>");
    let include_prev_line = e.line > 1;
    let max_len = e.line.to_string().len() + 2;

    println!("Error in /{}", file_path);
    println!("{}|", pad_right("", max_len, ' '));
    if include_prev_line {
        println!(
            "{}| {}",
            pad_right(&(e.line - 1).to_string(), max_len, ' '),
            lines[e.line - 2]
        );
    }
    println!("{}|", pad_right("", max_len, ' '));
    println!(
        "{}| {}",
        pad_right(&e.line.to_string(), max_len, ' '),
        lines[e.line - 1]
    );
    if let Some(cols) = e.cols {
        println!(
            "{}| {}{}",
            pad_right(" ", max_len, ' '),
            " ".repeat(cols.0 - 1),
            "^".repeat(cols.1 - cols.0)
        );
        println!();
        println!("|>|>{} {}", " ".repeat(cols.0), e);
    } else {
        println!(
            "{}| {}",
            pad_right("", max_len, ' '),
            "^".repeat(lines[e.line - 1].len())
        );
        println!();
        println!(">>>  {}", e);
    }
    // println!("{}| ", pad_right("..", max_len, ' '));
}

fn pad_right(s: &str, max_len: usize, c: char) -> String {
    s.chars()
        .chain(std::iter::repeat(c))
        .take(max_len)
        .collect()
}

pub struct Error {
    pub line: usize,
    pub kind: ErrorKind,
    pub cols: Option<(usize, usize)>,
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
            ErrorKind::UnexpectedToken(expected, actual) => {
                write!(
                    f,
                    "Unexpected token '{:?}'{}",
                    actual,
                    if expected.is_some() {
                        format!(", expected '{:?}'", expected.unwrap())
                    } else {
                        "".to_string()
                    }
                )
            }
            ErrorKind::Assignment => write!(f, "Invalid assignment"),
            ErrorKind::Import(file_name) => write!(f, "Could not import file: '{file_name}'"),
            ErrorKind::Runtime(s) => write!(f, "Runtime error: {}", s),
        }
    }
}

pub enum ErrorKind {
    UnexpectedCharacter,
    UnterminatedString,
    UnexpectedToken(Option<TokenKind>, TokenKind),
    Assignment,
    Import(String),
    Unknown,
    Runtime(String),
}
