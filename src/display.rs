pub struct Point {
    pub row: usize,
    pub col: usize,
}

pub struct Rect {
    pub width: u16,
    pub height: u16,
}

struct Cell {
    char: char,
    fg: String,
    bg: String,
}

impl Cell {
    pub fn new(char: char) -> Cell {
        let default = termion::color::Reset;
        Cell {
            char: char,
            fg: default.fg_str().to_string(),
            bg: default.bg_str().to_string(),
        }
    }
}

pub struct Display {
    content: Vec<Cell>,
    size: Rect,
}

impl Display {
    pub fn new(size: Rect, content: &String) -> Display {
        let mut display = Display {
            content: vec![],
            size: size,
        };
        display.set_content(content);
        display
    }

    pub fn rendered(&self) -> String {
        let mut rendered = String::from("");
        let mut prev_bg = termion::color::Reset.bg_str().to_string();
        let mut prev_fg = termion::color::Reset.fg_str().to_string();
        for line in self.content.chunks(self.size.width as usize) {
            for cell in line.iter() {
                if prev_bg != cell.bg {
                    rendered.push_str(cell.bg.as_str());
                    prev_bg = cell.bg.clone();
                }
                if prev_fg != cell.fg {
                    rendered.push_str(cell.fg.as_str());
                    prev_fg = cell.fg.clone();
                }
                if cell.char != '\n' {
                    rendered.push(cell.char);
                }
            }
            rendered.push_str(" \r\n");
        }
        rendered
    }

    pub fn set_content(&mut self, content: &String) {
        self.empty();

        let mut row = 0;
        let mut col = 0;
        for char in content[..].chars() {
            let pos = row * self.size.width + col;
            if pos as usize > self.content.len() - 1 {
                return;
            }
            self.content[pos as usize] = Cell::new(char);
            if char == '\n' || col == self.size.width {
                row += 1;
                col = 0;
            } else {
                col += 1;
            }
        }
    }

    fn empty(&mut self) {
        self.content = vec![];
        (1..self.size.width * self.size.height).for_each(|_| self.content.push(Cell::new(' ')));
    }

    pub fn highlight(
        &mut self,
        start: Point,
        end: Point,
        fg: Option<&String>,
        bg: Option<&String>,
    ) {
        let start_idx = start.row * self.size.width as usize + start.col;
        let end_idx = end.row * self.size.width as usize + end.col;

        for i in start_idx..end_idx {
            if i >= self.content.len() {
                return;
            }
            if let Some(color) = fg {
                self.content[i].fg = color.clone();
            }
            if let Some(color) = bg {
                self.content[i].bg = color.clone();
            }
        }
    }
}
