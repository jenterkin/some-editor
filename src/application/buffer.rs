use ropey::Rope;
use std::ops::Range;

type Selection = Range<usize>;
type Selections = Vec<Selection>;

pub struct Buffer {
    pub data: Rope,
    pub selections: Selections,
    scroll_col: Option<usize>,
}

impl Buffer {
    pub fn new(data: String) -> Buffer {
        return Buffer {
            data: Rope::from(data),
            selections: vec![0..1],
            scroll_col: None,
        };
    }

    pub fn len_lines(&mut self) -> usize {
        self.data.len_lines()
    }

    pub fn select_char_down(&mut self) {
        let selection = self.get_root_selection();
        let selection_line_idx = self.get_root_selection_line();
        let line_char_idx = self.data.line_to_char(selection_line_idx);
        let selection_col = if let Some(col) = self.scroll_col {
            col
        } else {
            let new_col = selection.start - line_char_idx;
            self.scroll_col = Some(new_col);
            new_col
        };
        let next_line_idx = selection_line_idx + 1;
        let next_line_char_idx = self.data.line_to_char(next_line_idx);
        let next_line_len = self.data.line(next_line_idx).len_chars();
        let new_start_pos = if selection_col < next_line_len {
            next_line_char_idx + selection_col
        } else {
            next_line_char_idx + next_line_len - 1
        };
        self.selections[0] = new_start_pos..new_start_pos + (selection.end - selection.start);
    }

    pub fn select_char_up(&mut self) {
        let selection = self.get_root_selection();
        let selection_line_idx = self.get_root_selection_line();
        let line_char_idx = self.data.line_to_char(selection_line_idx);
        let selection_col = if let Some(col) = self.scroll_col {
            col
        } else {
            let new_col = selection.start - line_char_idx;
            self.scroll_col = Some(new_col);
            new_col
        };
        let prev_line_idx = selection_line_idx - 1;
        let prev_line_char_idx = self.data.line_to_char(prev_line_idx);
        let prev_line_len = self.data.line(prev_line_idx).len_chars();
        let new_start_pos = if selection_col < prev_line_len {
            prev_line_char_idx + selection_col
        } else {
            prev_line_char_idx + prev_line_len - 1
        };
        self.selections[0] = new_start_pos..new_start_pos + (selection.end - selection.start);
    }

    pub fn select_char_left(&mut self) {
        if self.selections[0].start > 0 && self.selections[0].end > 0 {
            self.selections[0].start -= 1;
            self.selections[0].end -= 1;
            self.scroll_col = Some(self.get_root_selection_col());
        }
    }

    pub fn select_char_right(&mut self) {
        self.selections[0].start += 1;
        self.selections[0].end += 1;
        self.scroll_col = Some(self.get_root_selection_col());
    }

    pub fn get_root_selection(&self) -> Selection {
        self.selections[0].clone()
    }

    pub fn get_root_selection_line(&self) -> usize {
        self.data.char_to_line(self.get_root_selection().start)
    }

    pub fn get_root_selection_col(&self) -> usize {
        let selection = self.get_root_selection();
        let line_idx = self.get_root_selection_line();
        let line_pos = self.data.line_to_char(line_idx);
        selection.start - line_pos
    }
}
