use ncurses::*;
use uvm_log::*;


fn main() {
    let log = parse_file("sample.log");
    // let mut log_str = String::new();
    
    let mut tui = tui::Tui::init();
    for line in log {
        tui.content_buf.push_str(line.to_string().as_ref());
        tui.content_buf.push_str("\n");
    }
    
    // let mut i = 0;
    // for line in log_str.lines().take(tui.max_y as usize - 1) {
    //     addstr(line);
    //     addch('\n' as chtype);
    //     // i += 1;
    //     // if i == max_y - 1 {
    //     //     break;
    //     // }
    // }
    // tui.display_content();
    // tui.update_status();
    // tui.display_status();
    // let mut scroll_val = 0;
    // loop {
    //     // statusbar(tui.max_x, max_y, tui.cur_y as i32);
    //     getmaxyx(stdscr(), &mut tui.max_y, &mut tui.max_x);

    //     if scroll_val == 1 {
    //         scrl(1);
    //         mv(tui.max_y - 2, 0);
    //         addstr(log_str.lines().nth((tui.cur_y + tui.max_y - 1) as usize).unwrap());
    //     }
    //     else if scroll_val == -1 {
    //         scrl(-1);
    //         mv(0, 0);
    //         addstr(log_str.lines().nth(tui.cur_y as usize).unwrap());
    //     }


    //     let ch = getch();
    //     match ch {
    //         KEY_DOWN => {
    //             // if (tui.cur_y as i32) < tui.max_y {
    //                 tui.cur_y += 1;
    //             // }
    //             // scrl(1);
    //             scroll_val = 1;
    //         }
    //         KEY_UP => {
    //             // if tui.cur_y > 1 {
    //                 tui.cur_y -= 1;
    //             // }
    //             // scrl(-1);
    //             scroll_val = -1;
    //         }
    //         _ => {}
    //     }
    //     match std::char::from_u32(ch as u32).unwrap() {
    //         'q' => {
    //             break;
    //         }
    //         _ => {}
    //     }
    //     // clear();
    //     // mv(0, 0);
    // }
    // tui.refresh();
    // getch();
    tui.display();
    tui.exit();
}
