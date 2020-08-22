use uvm_log::*;

fn main() {
    let log = parse_file("sample.log");

    let mut tui = tui::Tui::init();
    for line in log {
        tui.content.push(line.to_string());
    }

    tui.display();
    tui.exit();
}
