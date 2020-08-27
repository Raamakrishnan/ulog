use uvm_log::*;
use clap::{App, Arg};


fn main() {
    let matches = App::new("uvm_log")
                        .version("0.1")
                        .arg(Arg::with_name("LOG")
                            .help("Log file")
                            .required(true))
                        .arg(Arg::with_name("id")
                            .long("id")
                            .takes_value(true)
                            .multiple(true)
                            .help("filter by id"))
                        .get_matches();

    let file = matches.value_of("LOG").unwrap();

    let filtered_ids : Option<Vec<&str>> = matches.values_of("id").map(|id| id.collect());

    let log = parse_file(file);
    let lines = log.into_iter();

    let filtered_lines = lines.filter(|line|
        match &filtered_ids {
            Some(ids) => {
                return ids.contains(&&line.id[..])
            },
            None => {
                return true
            },
        }
    );

    for line in filtered_lines {
        println!("{}", line);
    }

}
