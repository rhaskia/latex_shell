pub struct Editor {
    file: Vec<String>,
    str_file: String,
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
            str_file: String::new(),
            cursor: Cursor { line: 0, col: 0, max_col: 0 },
        }
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
            self.cursor.line -= 1;
            self.cursor.col = self.file[self.cursor.line].len();
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
        self.cursor.line += 1;
        self.cursor.col = 0;
        self.ensure_file_lines(self.cursor.line);
        self.file.insert(self.cursor.line, String::new());
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
