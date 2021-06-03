use ropey::Rope;
use std::array::IntoIter;
use std::collections::HashMap;
use std::iter::FromIterator;
// use std::ops::Range;

#[derive(Copy, Clone)]
pub struct Selection {
    start: usize,
    end: usize,
}

// type Selections = HashMap<usize, Selection>;
#[derive(Clone)]
pub struct Selections {
    root: usize,
    selections: HashMap<usize, Vec<Selection>>,
}

impl Selections {
    pub fn new(selections: HashMap<usize, Vec<Selection>>) -> Selections {
        Selections {
            root: 0,
            selections: selections,
        }
    }

    pub fn update_root_selection(&mut self, start_delta: isize, end_delta: isize) {
        let selection = self.selections.get_mut(&self.root).unwrap();

        if start_delta < 0 {
            selection[0].start -= (start_delta * -1) as usize
        } else {
            selection[0].start += start_delta as usize
        }

        if end_delta < 0 {
            selection[0].end -= (end_delta * -1) as usize
        } else {
            selection[0].end += end_delta as usize
        }
    }
}

pub struct Buffer {
    pub data: Rope,
    pub selections: Selections,
}

impl Buffer {
    pub fn new(data: String) -> Buffer {
        return Buffer {
            data: Rope::from(data),
            selections: Selections::new(HashMap::from_iter(IntoIter::new([(
                0,
                vec![Selection { start: 0, end: 1 }],
            )]))),
        };
    }

    pub fn len_lines(&mut self) -> usize {
        self.data.len_lines()
    }

    pub fn select_char_left(&mut self) {
        self.selections.update_root_selection(-1, -1);
    }

    pub fn select_char_down(&mut self) {}

    pub fn select_char_up(&mut self) {}

    pub fn select_char_right(&mut self) {
        self.selections.update_root_selection(1, 1);
    }
}
