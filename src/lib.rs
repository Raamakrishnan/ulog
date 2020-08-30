pub mod log;
pub mod parser;
// pub mod tui;
use std::path::Path;
use std::io::BufRead;
use std::path::PathBuf;

use log::Log;

/// Type-erased errors
pub type BoxError = std::boxed::Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>;

pub fn construct_log_line(severity_str : &str, file_str: &str, line_str: &str, time_str:&str, time_unit_str:&str, comp_str: &str, id_str: &str, msg_str : &str) -> Result<log::Line, BoxError> {
    let severity = severity_str.parse::<log::Severity>()?;
    let file = PathBuf::from(file_str);
    let line = line_str.parse::<u32>()?;
    let time = time_str.parse::<u64>()?;
    let time_unit: Option<log::TimeUnit>;
    if time_unit_str == "" {
        time_unit = None
    } else {
        time_unit = Some(time_unit_str.parse::<log::TimeUnit>()?);
    }
    let component = comp_str.to_string();
    let id = id_str.to_string();
    let message = msg_str.to_string();
    Ok(log::Line {
        severity,
        file,
        line,
        time,
        time_unit,
        component,
        id,
        message
    })
}

pub fn parse_file(path : impl AsRef<Path>) -> Log {
    let file = std::fs::File::open(path).expect("file open failed");
    let buf_reader = std::io::BufReader::new(file);

    let mut log = Log::new();

    for (i, line) in buf_reader.lines().enumerate() {
        let l = line.expect("line failed");
        let log_result = parser::parse_log_line(&l);
        match log_result {
            Ok((_, (severity_str, file_str, line_str, time_str, time_unit_str, comp_str, id_str, msg_str))) => {
                let log_line = match construct_log_line(severity_str, file_str, line_str, time_str, time_unit_str, comp_str, id_str, msg_str) {
                    Ok(line) => line,
                    Err(e) => {
                        println!("line {} ignored due to error: {}", i, e.to_string());
                        continue;
                    },
                };
                log.push(log_line)
            },// Ignore parsing errors
            Err(e) => println!("line {} ignored due to error: {}", i, e.to_string()),
        }
    }

    return log;
}