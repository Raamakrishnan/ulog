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
    let ids = matches.values_of("id");

    if let Some(v_ids) = ids {
        let v_ids: Vec<&str> = v_ids.collect();
        let log = parse_file(file);

        let filtered_lines = log.into_iter();
        let filtered_lines = filtered_lines.into_iter().filter(|y| v_ids.contains(&&y.id[..]));

        for line in filtered_lines {
            println!("{}", line);
        }
    }

}
