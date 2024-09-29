use std::fmt::Display;

pub fn print_error(e: Error, file_path: &str, source: &str) {
    let lines = source.lines().collect::<Vec<&str>>();

    let include_prev_line = e.line > 1;
    println!("include: {}", include_prev_line);
    let max_len = e.line.to_string().len() + 2;

    println!("Error in /{}", file_path);
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
            " ".repeat(cols.0),
            "^".repeat(cols.1 - cols.0)
        );
    } else {
        println!(
            "{}| {}",
            pad_right("", max_len, ' '),
            "^".repeat(lines[e.line - 1].len())
        );
    }
    println!("{}| ", pad_right("..", max_len, ' '));
    println!(">>>   {}", e);
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
            ErrorKind::UnexpectedToken => write!(f, "Unexpected token"),
            ErrorKind::Assignment => write!(f, "Invalid assignment"),
            ErrorKind::Import(file_name) => write!(f, "Could not import file: '{file_name}'"),
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
