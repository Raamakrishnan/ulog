use ncurses::*;
use uvm_log::*;

fn main() {
    let log = parse_file("sample.log");
    let mut log_str = String::new();
    for line in log {
        log_str.push_str(line.to_string().as_ref());
        log_str.push_str("\n");
    }

    let mut max_x = 0;
    let mut max_y = 0;
    let mut cur_y = 0;

    initscr();
    raw();
    noecho();
    keypad(stdscr(), true);

    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    loop {
        let mut i = 0;
        for line in log_str.lines().skip(cur_y) {
            addstr(line);
            addch('\n' as chtype);
            i += 1;
            if i == max_y - 1 {
                break;
            }
        }
        let ch = getch();
        match ch {
            KEY_DOWN => {
                cur_y += 1;
            }
            KEY_UP => {
                if cur_y > 1 {
                    cur_y -= 1;
                }
            }
            _ => {}
        }
        match std::char::from_u32(ch as u32).unwrap() {
            'q' => {
                break;
            }
            _ => {}
        }
        mv(0, 0);
    }
    endwin();
}
