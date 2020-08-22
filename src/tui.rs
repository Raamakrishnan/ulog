use ncurses::*;

#[derive(Debug)]
pub struct Tui {
    pub screenrows: i32,
    pub screencols: i32,
    pub row_offset: i32,
    pub col_offset: i32,

    pub content_win: WINDOW,
    pub status_bar: WINDOW,

    pub content: Vec<String>,
    pub status_buf: String,
}

impl Tui {

    pub fn init() -> Tui {
        initscr();
        raw();
        noecho();
        keypad(stdscr(), true);
        // curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        let mut max_y = 0;
        let mut max_x = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        let (content_win, status_bar) = Tui::create_windows(max_y, max_x);
        Tui {
            screencols: max_x,
            screenrows: max_y,
            row_offset: 0,
            col_offset: 0,
            content_win,
            status_bar,
            content: Vec::new(),
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
        for (i, line) in self.content.iter().take(lines).enumerate() {
            let line = format!("{} {}", i, line);
            waddstr(self.content_win, line.as_ref());
            if i < lines - 1 {
                waddch(self.content_win, '\n' as chtype);
            }
        }
    }

    pub fn display_content_line(&self, line: usize) {
        let line_str = &self.content[line];
        let line_str = format!("{} {}", line, line_str);
        waddstr(self.content_win, line_str.as_ref());
    }

    pub fn update_status(&mut self, debug: &str) {
        self.status_buf = format!("{}/{} {}", self.row_offset, self.content.len(), debug);
    }

    pub fn display_status(&self) {
        wmove(self.status_bar, 0, 0);
        waddstr(self.status_bar, self.status_buf.as_ref());
        wclrtoeol(self.status_bar);
    }

    fn scroll_down(&mut self) {
        self.row_offset += 1;
        wscrl(self.content_win, 1);
        wmove(self.content_win, self.screenrows - 2, 0);
        self.display_content_line((self.row_offset + self.screenrows - 2) as usize);
    }

    fn scroll_up(&mut self) {
        self.row_offset -= 1;
        wscrl(self.content_win, -1);
        wmove(self.content_win, 0, 0);
        self.display_content_line(self.row_offset as usize);
    }

    pub fn display(&mut self) {
        self.display_content(self.screenrows as usize - 1);
        self.update_status("Welcome");
        self.display_status();
        loop {
            self.refresh();
            let ch = getch();
            match ch {
                KEY_DOWN => {
                    if self.row_offset < ((self.content.len() as i32) - self.screenrows + 1) {
                        self.scroll_down();
                    }
                },
                KEY_UP => {
                    if self.row_offset > 0 {
                        self.scroll_up();
                    }
                },
                _ => {},
            }
            match std::char::from_u32(ch as u32).unwrap() {
                'q' => break,
                _ => {},
            }
            self.update_status("");
            self.display_status();
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