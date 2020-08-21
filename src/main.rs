use std::io::BufRead;
use uvm_log::*;

fn main() {
    let file = std::fs::File::open("sample.log").expect("file open failed");
    let buf_reader = std::io::BufReader::new(file);
    for line in buf_reader.lines() {
        let l = line.expect("line failed");
        let log = parser::parse_log_line(&l);
        match log {
            Ok((_, log2)) => {
                println!("{}", log2);
            },
            Err(e) => println!("parsing error {:?}", e.to_string())
        };
    }
}
