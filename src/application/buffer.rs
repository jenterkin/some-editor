use ropey::Rope;
use std::ops::Range;

type Selection = Range<usize>;
type Selections = Vec<Selection>;

pub struct Buffer {
    pub data: Rope,
    pub selections: Selections,
}

impl Buffer {
    pub fn new(data: String) -> Buffer {
        return Buffer {
            data: Rope::from(data),
            selections: vec![0..1],
        };
    }

    pub fn len_lines(&mut self) -> usize {
        self.data.len_lines()
    }

    pub fn select_char_left(&mut self) {
        self.selections[0].start -= 1;
        self.selections[0].end -= 1;
    }

    pub fn select_char_down(&mut self) {}

    pub fn select_char_up(&mut self) {}

    pub fn select_char_right(&mut self) {
        self.selections[0].start += 1;
        self.selections[0].end += 1;
    }
}
