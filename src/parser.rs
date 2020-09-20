use nom::branch::alt;
use nom::sequence::tuple;
use nom::{bytes::complete::is_not, bytes::complete::tag};


fn not_whitespace(i: &str) -> nom::IResult<&str, &str> {
    is_not(" \t")(i)
}

fn parse_log_severity(i: &str) -> nom::IResult<&str, &str> {
    alt((
        tag("UVM_INFO"),
        tag("UVM_WARNING"),
        tag("UVM_ERROR"),
        tag("UVM_FATAL"),
    ))(i)
}

fn parse_file_line(i: &str) -> nom::IResult<&str, (&str, &str)> {
    let (i, (file, _, line, _)) = tuple((
        is_not("("),
        nom::character::complete::char('('),
        nom::character::complete::digit1,
        nom::character::complete::char(')'),
    ))(i)?;
    Ok((i, (file, line)))
}

fn parse_time(i: &str) -> nom::IResult<&str, &str> {
    let (i, (time, _)) = tuple((
        nom::bytes::complete::take_until(":"),
        nom::character::complete::char(':'),
    ))(i)?;
    Ok((i, time))
}

fn parse_id(i: &str) -> nom::IResult<&str, &str> {
    let (i, (_, id, _)) = tuple((
        nom::character::complete::char('['),
        nom::bytes::complete::take_until("]"),
        nom::character::complete::char(']'),
    ))(i)?;
    Ok((i, id))
}

pub fn parse_log_line(i: &str) -> nom::IResult<&str, (&str, &str, &str, &str, &str, &str)> {
    let (i, severity) = parse_log_severity(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, (file, line)) = parse_file_line(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, _) = nom::character::complete::char('@')(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, _) = parse_time(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, comp) = not_whitespace(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, id) = parse_id(i)?;
    let (i, _) = nom::character::complete::space1(i)?;

    Ok(("", (severity, file, line, comp, id, i)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_log_severity() {
        assert_eq!(
            parse_log_severity("UVM_INFO asacs"),
            Ok((" asacs", "UVM_INFO"))
        );
        assert_eq!(
            parse_log_severity("UVM_WARNING 312"),
            Ok((" 312", "UVM_WARNING"))
        );
        assert_eq!(
            parse_log_severity("UVM_ERROR @ ("),
            Ok((" @ (", "UVM_ERROR"))
        );
        assert_eq!(
            parse_log_severity("UVM_FATAL /1.1"),
            Ok((" /1.1", "UVM_FATAL"))
        );
    }

    #[test]
    fn test_parse_file_line() {
        assert_eq!(
            parse_file_line("sample/sample.sv(98) 4245"),
            Ok((" 4245", ("sample/sample.sv", "98")))
        );
    }

    #[test]
    fn test_parse_id() {
        assert_eq!(parse_id("[abc] adce"), Ok((" adce", "abc")));
        assert_eq!(parse_id("[tag2] 1014"), Ok((" 1014", "tag2")));
    }

    #[test]
    fn test_parse_time() {
        assert_eq!(parse_time("12345ns: adce"), Ok((" adce", "12345ns")));
    }

    #[test]
    fn test_parse_line() {
        let log1 = (
            "UVM_FATAL",
            "/home/runner/env.svh",
            "46",
            // "25",
            // "",
            "uvm_test_top.jb_env.jb_fc",
            "id1",
            "GREEN BUBBLE_GUM 7"
        );
        let log2 = (
            "UVM_FATAL",
            "/home/runner/env.svh",
            "46",
            // "25",
            // "ns",
            "uvm_test_top.jb_env.jb_fc",
            "id1",
            "GREEN BUBBLE_GUM 7"
        );
        assert_eq!(parse_log_line("UVM_FATAL /home/runner/env.svh(46) @ 25: uvm_test_top.jb_env.jb_fc [id1] GREEN BUBBLE_GUM 7"), Ok(("", log1)));
        assert_eq!(parse_log_line("UVM_FATAL /home/runner/env.svh(46) @ 25ns: uvm_test_top.jb_env.jb_fc [id1] GREEN BUBBLE_GUM 7"), Ok(("", log2)));
    }
}
