use std::io::BufRead;
use uvm_log::*;

fn main() {
    let log = parse_file("sample.log");
    for line in log {
        println!("{}", line);
    }
}
