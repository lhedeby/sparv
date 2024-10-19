use std::{fs::OpenOptions, io::Write};

pub fn log(text: String) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("./log")
        .unwrap();
    writeln!(file, "{}", text).unwrap();
    file.flush().unwrap();
}

#[macro_export]
macro_rules! log {
    ($($args: tt)*) => {
        logger::log(format!($($args)*))
    }
}
