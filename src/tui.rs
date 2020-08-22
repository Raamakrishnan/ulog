use ncurses::*;

#[derive(Debug)]
pub struct Tui {
    pub max_y: i32,
    pub max_x: i32,
    pub cur_y: i32,
    pub cur_x: i32,

    pub content_win: WINDOW,
    pub status_bar: WINDOW,

    pub content_buf: String,
    pub status_buf: String,
}

impl Tui {

    pub fn init() -> Tui {
        initscr();
        raw();
        noecho();
        keypad(stdscr(), true);
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        let mut max_y = 0;
        let mut max_x = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        let (content_win, status_bar) = Tui::create_windows(max_y, max_x);
        Tui {
            max_x,
            max_y,
            cur_y: 0,
            cur_x: 0,
            content_win,
            status_bar,
            content_buf: String::new(),
            status_buf: String::new(),
        }
    }

    fn create_windows(max_y: i32, max_x: i32) -> (WINDOW, WINDOW) {
        // 2 windows, 1 for content, 1 for status bar
        let content_win = newwin(max_y - 1, max_x, 0, 0);
        scrollok(content_win, true);
        keypad(content_win, true);
        let status_bar = newwin(1, max_x, max_y - 1, 0);
        (content_win, status_bar)
    }

    pub fn display_content(&self, lines: usize) {
        for (i, line) in self.content_buf.lines().take(lines).enumerate() {
            let line = format!("{} {}", i, line);
            waddstr(self.content_win, line.as_ref());
            if i < lines - 1 {
                waddch(self.content_win, '\n' as chtype);
            }
        }
    }

    pub fn display_content_line(&self, line: usize) {
        let line_str = self.content_buf.lines().nth(line).unwrap();
        let line_str = format!("{} {}", line, line_str);
        waddstr(self.content_win, line_str.as_ref());
    }

    pub fn update_status(&mut self) {
        self.status_buf = format!("{}/{}", self.cur_y, self.max_y);
    }

    pub fn display_status(&self) {
        wmove(self.status_bar, 0, 0);
        waddstr(self.status_bar, self.status_buf.as_ref());
    }

    pub fn display(&mut self) {
        self.display_content(self.max_y as usize );
        let mut scroll_val = 0;
        loop {
            if scroll_val == 1 {
                self.cur_y += 1;
                wscrl(self.content_win, 1);
                wmove(self.content_win, self.max_y - 2, 0);
                self.display_content_line((self.cur_y + self.max_y - 1) as usize);
            }
            else if scroll_val == -1 {
                self.cur_y -= 1;
                wscrl(self.content_win, -1);
                wmove(self.content_win, 0, 0);
                self.display_content_line(self.cur_y as usize);
            }
            self.update_status();
            self.display_status();
            self.refresh();
            let ch = getch();
            match ch {
                KEY_DOWN => {
                    if self.cur_y < self.max_y {
                        scroll_val = 1;
                    }
                },
                KEY_UP => {
                    if self.cur_y > 1 {
                        scroll_val = -1;
                    }
                },
                _ => {},
            }
            match std::char::from_u32(ch as u32).unwrap() {
                'q' => break,
                _ => {},
            }
        }
    }

    pub fn refresh(&self) {
        refresh();
        wrefresh(self.content_win);
        wrefresh(self.status_bar);
    }

    pub fn exit(&self) {
        endwin();
    }

}