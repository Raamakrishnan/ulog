pub mod log;
pub mod parser;
pub mod tui;
use std::path::Path;
use std::io::BufRead;

use log::Log;

/// Type-erased errors
pub type BoxError = std::boxed::Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>;

pub fn parse_file(path : impl AsRef<Path>) -> Log {
    let file = std::fs::File::open(path).expect("file open failed");
    let buf_reader = std::io::BufReader::new(file);

    let mut log = Log::new();

    for line in buf_reader.lines() {
        let l = line.expect("line failed");
        let (_, log_line) = parser::parse_log_line(&l).unwrap();
        log.push(log_line);
    }

    return log;
}