use clap::{App, Arg};
use ulog::*;

fn main() {
    let matches = App::new("ulog")
        .version("0.1")
        .arg(Arg::with_name("LOG").help("Log file").required(true))
        .arg(
            Arg::with_name("id")
                .long("id")
                .takes_value(true)
                .multiple(true)
                .help("filter by id"),
        )
        .arg(
            Arg::with_name("severity")
                .long("severity")
                .takes_value(true)
                .multiple(true)
                .help("Filter by severity (info, warning, error, fatal)"),
        )
        .get_matches();

    let file = matches.value_of("LOG").unwrap();

    let filtered_ids: Option<Vec<&str>> = matches.values_of("id").map(|id| id.collect());
    let filtered_severity: Option<Vec<&str>> =
        matches.values_of("severity").map(|sev| sev.collect());

    let filtered_severity: Option<Vec<log::Severity>> = filtered_severity.map(|vsev| {
        vsev.into_iter()
            .filter_map(|sev| sev.parse::<log::Severity>().ok())
            .collect()
    });

    let log = parse_file(file);
    let lines = log.into_iter();

    let filtered_lines = lines.filter(|line| {
        if let (Some(ids), Some(sev)) = (&filtered_ids, &filtered_severity) {
            return ids.contains(&&line.id[..]) && sev.contains(&&line.severity);
        } else if let Some(ids) = &filtered_ids {
            return ids.contains(&&line.id[..]);
        } else if let Some(sev) = &filtered_severity {
            return sev.contains(&&line.severity);
        } else {
            return false;
        }
    });

    for line in filtered_lines {
        println!("{}", line);
    }
}
