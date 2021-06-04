use super::traits::View as ViewTrait;
use crate::application::buffer::Buffer;
use crate::application::modes::Modes;
use crate::highlight::Highlighter;
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

pub struct Terminal {
    top: usize,
    output: BufWriter<RawTerminal<AlternateScreen<Stdout>>>,
    last_position: CursorPosition,
    position: CursorPosition,
    pub command: String,
    mode: Modes,
    highlighter: Highlighter,
    processed_buffer: Rope,
}

impl Terminal {
    pub fn new() -> Terminal {
        return Terminal {
            output: BufWriter::with_capacity(
                1_048_576,
                AlternateScreen::from(stdout()).into_raw_mode().unwrap(),
            ),
            top: 0,
            // TODO(jenterkin): `last_position` should probably be optional
            last_position: CursorPosition { row: 1, col: 1 },
            position: CursorPosition { row: 1, col: 1 },
            command: String::from(""),
            mode: Modes::Normal,
            highlighter: Highlighter::new(),
            processed_buffer: Rope::from(""),
        };
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
        write!(self.output, "{}", termion::cursor::Hide).unwrap();
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
        if self.top > 0 {
            self.top -= 1;
        }
    }

    fn scroll_down(&mut self, len_lines: usize) {
        // why is it `- 2`?
        if self.top < len_lines - 2 {
            self.top += 1;
        }
    }

    fn render(&mut self, buffer: &Buffer, command: &String) {
        if self.processed_buffer.len_chars() == 0 {
            self.processed_buffer =
                Rope::from(self.highlighter.highlight(&buffer.data.clone().to_string()));
        }

        let mut row = 1;
        let term_height = termion::terminal_size().unwrap().1;
        for i in self.top..self.top + term_height as usize - 1 {
            write!(
                self.output,
                "{}{}{}",
                termion::cursor::Goto(1, row),
                termion::clear::CurrentLine,
                self.processed_buffer.line(i).to_string()
            )
            .unwrap();
            row += 1;
        }

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
