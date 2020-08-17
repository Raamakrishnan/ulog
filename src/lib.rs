use std::path::PathBuf;
use nom::branch::alt;
use nom::{bytes::complete::tag, bytes::complete::is_not, combinator::value};
use nom::sequence::tuple;

/// Type-erased errors
pub type BoxError = std::boxed::Box<dyn
    std::error::Error
    + std::marker::Send
    + std::marker::Sync
>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LogSeverity {
    INFO,
    WARNING,
    ERROR,
    FATAL,
}

impl std::str::FromStr for LogSeverity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UVM_INFO" => Ok(LogSeverity::INFO),
            "UVM_WARNING" => Ok(LogSeverity::WARNING),
            "UVM_ERROR" => Ok(LogSeverity::ERROR),
            "UVM_FATAL" => Ok(LogSeverity::FATAL),
            _ => Err(format!("'{}' is not a valid severity", s))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Log {
    severity: LogSeverity,
    file: PathBuf,
    line: u32,
    time: u64,
    component: String,
    id: String,
    message: String,
}

pub mod parser {
    use super::*;

    fn not_whitespace(i: &str) -> nom::IResult<&str, &str> {
        is_not(" \t")(i)
    }

    fn parse_log_severity(i: &str) -> nom::IResult<&str, LogSeverity> {
        alt((
            value(LogSeverity::INFO, tag("UVM_INFO")),
            value(LogSeverity::WARNING, tag("UVM_WARNING")),
            value(LogSeverity::ERROR, tag("UVM_ERROR")),
            value(LogSeverity::FATAL, tag("UVM_FATAL")),
        ))(i)
    }

    fn parse_file_line(i: &str) -> nom::IResult<&str, (PathBuf, u32)> {
        let (i, (file, _, line, _)) = tuple((
            is_not("("),
            nom::character::complete::char('('),
            nom::character::complete::digit1,
            nom::character::complete::char(')'),
        ))(i)?;
        Ok((i, (PathBuf::from(file), line.parse::<u32>().unwrap())))
    }

    fn parse_id(i: &str) -> nom::IResult<&str, &str> {
        let(i , (_, id, _)) = tuple((
            nom::character::complete::char('['),
            nom::bytes::complete::take_until("]"),
            nom::character::complete::char(']'),
        ))(i)?;
        Ok((i, id))
    }

    pub fn parse_log_line(i: &str) -> nom::IResult<&str, Log> {
        let (i, severity) = parse_log_severity(i)?;
        let (i, _) = nom::character::complete::space1(i)?;
        let (i, (file, line)) = parse_file_line(i)?;
        let (i, _) = nom::character::complete::space1(i)?;
        let (i, _) = nom::character::complete::char('@')(i)?;
        let (i, _) = nom::character::complete::space1(i)?;
        let (i, time) = nom::character::complete::digit1(i)?;
        let (i, _) = nom::character::complete::char(':')(i)?;
        let (i, _) = nom::character::complete::space1(i)?;
        let (i, comp) = not_whitespace(i)?;
        let (i, _) = nom::character::complete::space1(i)?;
        let (i, id) = parse_id(i)?;
        let (i, _) = nom::character::complete::space1(i)?;
        Ok(("", Log {
            file: file,
            line: line,
            id: id.to_string(),
            component: comp.to_string(),
            time: time.parse::<u64>().unwrap(),
            severity: severity,
            message: i.to_string()
        }))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse_log_severity() {
            assert_eq!(parse_log_severity("UVM_INFO asacs"), Ok((" asacs", LogSeverity::INFO)));
            assert_eq!(parse_log_severity("UVM_WARNING 312"), Ok((" 312", LogSeverity::WARNING)));
            assert_eq!(parse_log_severity("UVM_ERROR @ ("), Ok((" @ (", LogSeverity::ERROR)));
            assert_eq!(parse_log_severity("UVM_FATAL /1.1"), Ok((" /1.1", LogSeverity::FATAL)));
        }

        #[test]
        fn test_parse_file_line() {
            assert_eq!(parse_file_line("sample/sample.sv(98) 4245"), Ok((" 4245", (PathBuf::from("sample/sample.sv"), 98))));
        }

        #[test]
        fn test_parse_id() {
            assert_eq!(parse_id("[abc] adce"), Ok((" adce", "abc")));
            assert_eq!(parse_id("[tag2] 1014"), Ok((" 1014", "tag2")));
        }

        #[test]
        fn test_parse_line() {
            let log = Log {
                id: "id1".to_string(),
                component: "uvm_test_top.jb_env.jb_fc".to_string(),
                file: PathBuf::from("/home/runner/env.svh"),
                line: 46,
                message: "GREEN BUBBLE_GUM 7".to_string(),
                severity: LogSeverity::FATAL,
                time: 25
            };
            assert_eq!(parse_log_line("UVM_FATAL /home/runner/env.svh(46) @ 25: uvm_test_top.jb_env.jb_fc [id1] GREEN BUBBLE_GUM 7"), Ok(("", log)));
        }

    }

}