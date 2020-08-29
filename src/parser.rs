use nom::branch::alt;
use nom::sequence::tuple;
use nom::{bytes::complete::is_not, bytes::complete::tag, combinator::value};
use std::path::PathBuf;

use super::log::*;

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

fn parse_time(i: &str) -> nom::IResult<&str, (u64, &str)> {
    let (i, (time, unit, _)) = tuple((
        nom::character::complete::digit1,
        nom::character::complete::alpha0,
        nom::character::complete::char(':'),
    ))(i)?;
    Ok((i, (time.parse::<u64>().unwrap(), unit)))
}

fn parse_id(i: &str) -> nom::IResult<&str, &str> {
    let (i, (_, id, _)) = tuple((
        nom::character::complete::char('['),
        nom::bytes::complete::take_until("]"),
        nom::character::complete::char(']'),
    ))(i)?;
    Ok((i, id))
}

pub fn parse_log_line(i: &str) -> nom::IResult<&str, LogLine> {
    let (i, severity) = parse_log_severity(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, (file, line)) = parse_file_line(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, _) = nom::character::complete::char('@')(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    // let (i, time) = nom::character::complete::digit1(i)?;
    // let (i, _) = nom::character::complete::char(':')(i)?;
    let (i, (time, time_unit_str)) = parse_time(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, comp) = not_whitespace(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, id) = parse_id(i)?;
    let (i, _) = nom::character::complete::space1(i)?;

    let time_unit = time_unit_str.parse::<LogTimeUnit>();

    Ok((
        "",
        LogLine {
            file: file,
            line: line,
            id: id.to_string(),
            component: comp.to_string(),
            time: time,
            time_unit: time_unit.ok(),
            severity: severity,
            message: i.to_string(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_log_severity() {
        assert_eq!(
            parse_log_severity("UVM_INFO asacs"),
            Ok((" asacs", LogSeverity::INFO))
        );
        assert_eq!(
            parse_log_severity("UVM_WARNING 312"),
            Ok((" 312", LogSeverity::WARNING))
        );
        assert_eq!(
            parse_log_severity("UVM_ERROR @ ("),
            Ok((" @ (", LogSeverity::ERROR))
        );
        assert_eq!(
            parse_log_severity("UVM_FATAL /1.1"),
            Ok((" /1.1", LogSeverity::FATAL))
        );
    }

    #[test]
    fn test_parse_file_line() {
        assert_eq!(
            parse_file_line("sample/sample.sv(98) 4245"),
            Ok((" 4245", (PathBuf::from("sample/sample.sv"), 98)))
        );
    }

    #[test]
    fn test_parse_id() {
        assert_eq!(parse_id("[abc] adce"), Ok((" adce", "abc")));
        assert_eq!(parse_id("[tag2] 1014"), Ok((" 1014", "tag2")));
    }

    #[test]
    fn test_parse_line() {
        let log = LogLine {
            id: "id1".to_string(),
            component: "uvm_test_top.jb_env.jb_fc".to_string(),
            file: PathBuf::from("/home/runner/env.svh"),
            line: 46,
            message: "GREEN BUBBLE_GUM 7".to_string(),
            severity: LogSeverity::FATAL,
            time: 25,
            time_unit: None,
        };
        assert_eq!(parse_log_line("UVM_FATAL /home/runner/env.svh(46) @ 25: uvm_test_top.jb_env.jb_fc [id1] GREEN BUBBLE_GUM 7"), Ok(("", log)));
    }
}
