pub struct Editor {
    file: Vec<String>,
    cursor: Cursor,
}

#[derive(Debug, Copy, Clone)]
pub struct Cursor {
    pub line: usize,
    pub col: usize,
    pub max_col: usize,
}

impl Editor {
    pub fn new() -> Self {
        Editor { 
            file: Vec::new(),
            cursor: Cursor { line: 0, col: 0, max_col: 0 },
        }
    }

    pub fn paste(&mut self, to_paste: String) {
        println!("PASTED");
        // if to_paste == String::new() { return; }
        // let (start, end) = self.file[self.cursor.line].split_at(self.cursor.col);
        // let inserted = format!("{start}{to_paste}{end}");
        // let mut lines = inserted.lines();
        // let first = lines.next(); 
        //self.file[self.cursor.line] = first.unwrap().to_string();
    }

    pub fn get_file(&mut self) -> Vec<String> {
        self.file.clone()
    }

    pub fn get_cursor(&self) -> Cursor {
        self.cursor
    }

    pub fn backspace(&mut self) {
        // Start of file
        if self.cursor.line == 0 && self.cursor.col == 0 {
            return;
        }
        // Backspace at start of line -> wraps
        if self.cursor.col == 0 {
            let line_to_move = self.file.remove(self.cursor.line);
            self.cursor.line -= 1;
            self.cursor.col = self.file[self.cursor.line].len();
            self.file[self.cursor.line].push_str(&line_to_move);
            return;
        }
        self.file[self.cursor.line].remove(self.cursor.col - 1);
        self.cursor.col -= 1;
    }

    pub fn push(&mut self, c: char) {
        self.ensure_file_lines(self.cursor.line);
        // If the cursor is at the end of the line insert panics
        if self.cursor.col + 1 >= self.file[self.cursor.line].len() {
            self.file[self.cursor.line].push(c)
        } else {
            self.file[self.cursor.line].insert(self.cursor.col, c);
        }
        self.cursor.col += 1;
    }

    pub fn new_line(&mut self) {
        let split = self.file[self.cursor.line].split_at(self.cursor.col);
        let (start, end) = (split.0.to_string(), split.1.to_string());
        self.file[self.cursor.line] = start;
        self.cursor.line += 1;
        self.cursor.col = 0;
        self.ensure_file_lines(self.cursor.line);
        self.file.insert(self.cursor.line, end);
    }

    pub fn ensure_file_lines(&mut self, lines: usize) {
        if self.file.len() > (lines + 1) {
            return;
        }
        self.file.extend(vec![String::new(); (lines + 1) - self.file.len()]);
    }

    pub fn cursor_up(&mut self) {
        if self.cursor.line != 0 {
            self.cursor.max_col = self.cursor.col;
            self.cursor.line -= 1;
            self.cursor.col = self.cursor.max_col.min(self.file[self.cursor.line].len());
        }
    }

    pub fn cursor_down(&mut self) {
        if (self.cursor.line + 2) >= self.file.len() {
            return;
        }
        self.cursor.line += 1
    }

    pub fn cursor_left(&mut self) {
        // wrapping
        if self.cursor.col == 0 {
            if self.cursor.line == 0 {
                return;
            } // start of file
            self.cursor.line -= 1;
            self.cursor.col = self.file[self.cursor.line].len();
            return;
        }
        // normal movement
        self.cursor.col -= 1;
    }

    pub fn cursor_right(&mut self) {
        // wrapping
        if self.cursor.col >= self.file[self.cursor.line].len() {
            if (self.cursor.line + 2) >= self.file.len() {
                return;
            } // EOF
            self.cursor.col = 0;
            self.cursor.line += 1;
            return;
        }
        // normal movement
        self.cursor.col += 1;
    }
}
