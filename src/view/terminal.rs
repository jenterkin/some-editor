use super::traits::View as ViewTrait;
use crate::application::modes::Modes;
use ropey::Rope;
use std::io::{stdout, BufWriter, Stdout, Write};
use termion;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use termion::terminal_size;

#[derive(Clone)]
struct CursorPosition {
    row: u16,
    col: u16,
}

pub struct View {
    pub top: usize,
}

pub struct Terminal {
    view: View,
    output: BufWriter<RawTerminal<AlternateScreen<Stdout>>>,
    last_position: CursorPosition,
    position: CursorPosition,
    pub command: String,
    mode: Modes,
}

impl Terminal {
    pub fn new() -> Terminal {
        return Terminal {
            output: BufWriter::with_capacity(
                1_048_576,
                AlternateScreen::from(stdout()).into_raw_mode().unwrap(),
            ),
            view: View { top: 0 },
            // TODO(jenterkin): `last_position` should probably be optional
            last_position: CursorPosition { row: 1, col: 1 },
            position: CursorPosition { row: 1, col: 1 },
            command: String::from(""),
            mode: Modes::Normal,
        };
    }

    fn write_visible_lines(&mut self, data: &Rope) {
        let start = self.view.top;
        let height = terminal_size().unwrap().1 as usize;
        let num_lines = data.len_lines();
        let end = if (num_lines - start) > height as usize {
            self.view.top + height as usize
        } else {
            num_lines
        };

        let mut row = 1;
        for line_num in start..end {
            if let Some(line) = data.line(line_num).as_str() {
                row += 1;
                write!(self.output, "{}{}", line, termion::cursor::Goto(1, row)).unwrap();
            }
        }
    }

    fn update_position(&mut self, row: u16, col: u16) {
        self.position.row = row;
        self.position.col = col;
        write!(
            self.output,
            "{}",
            termion::cursor::Goto(self.position.col, self.position.row)
        )
        .unwrap();
    }
}

impl ViewTrait for Terminal {
    fn start(&mut self) {
        self.update_position(self.position.col, self.position.row);
    }

    fn change_mode(&mut self, mode: Modes) {
        // cleanup
        match self.mode {
            Modes::Command => {
                write!(self.output, "{}", termion::clear::CurrentLine).unwrap();
                self.position = self.last_position.clone();
                self.update_position(self.position.row, self.position.col);
                self.command = String::from("");
            }
            _ => {}
        }

        match mode {
            Modes::Command => {
                self.last_position = self.position.clone();
                let last_row = termion::terminal_size().unwrap().0;
                self.update_position(last_row, 1);
                self.position.col += 1;
                self.write_char(':');
                self.mode = Modes::Command
            }
            _ => {}
        }
    }

    fn write_char(&mut self, char: char) {
        match char {
            '\n' => {
                self.position.col = 1;
                self.position.row = self.position.row + 1;
            }
            _ => {
                self.position.col += 1;
            }
        }
        write!(self.output, "{}", char).unwrap();
        self.update_position(self.position.row, self.position.col);
    }

    fn scroll_up(&mut self) {
        if self.view.top > 0 {
            self.view.top -= 1;
        }
    }

    fn scroll_down(&mut self) {
        // TODO(jenterkin): stop at last line of buffer
        self.view.top += 1;
    }

    fn render(&mut self, data: &Rope, command: &String) {
        write!(
            self.output,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        )
        .unwrap();
        self.write_visible_lines(data);
        self.update_position(self.position.row, self.position.col);
        self.output.flush().unwrap();

        match self.mode {
            Modes::Command => {
                let last_line = termion::terminal_size().unwrap().1;
                self.position.row = last_line;
                write!(
                    self.output,
                    "{}:{}",
                    termion::cursor::Goto(1, last_line),
                    &command
                )
                .unwrap();
            }
            _ => {}
        }

        self.output.flush().unwrap();
    }
}
