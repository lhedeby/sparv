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
    // if let Some(cols) = e.cols {
        println!(
            "{}| {}{}",
            pad_right(" ", max_len, ' '),
            // TODO: Prevents err but probably isnt correct
            if e.start > 0 {
                " ".repeat(e.start - 1)
            } else {
                "".to_string()
            },
            "^".repeat(e.end - e.start)
        );
        println!();
        println!("|>|>{} {}", " ".repeat(e.start), e);
}

fn pad_right(s: &str, max_len: usize, c: char) -> String {
    s.chars()
        .chain(std::iter::repeat(c))
        .take(max_len)
        .collect()
}

pub struct Error {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub msg: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
