use super::traits::View as ViewTrait;
use crate::application::buffer::Buffer;
use crate::application::modes::Modes;
use crate::display::{Display, Point, Rect};
use crate::highlight::Highlighter;
use ropey::Rope;
use std::io::{stdout, BufWriter, Stdout, Write};
use termion;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

fn char_idxs_to_points(
    start: usize,
    end: usize,
    offset: usize,
    content: ropey::RopeSlice,
) -> (Point, Point) {
    let start_idx = start - offset;
    let end_idx = end - offset;

    let start_row = content.char_to_line(start_idx);
    let end_row = content.char_to_line(end_idx);

    let start_col = start_idx - content.line_to_char(start_row);
    let end_col = end_idx - content.line_to_char(end_row);

    (
        Point {
            row: start_row,
            col: start_col,
        },
        Point {
            row: end_row,
            col: end_col,
        },
    )
}

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

    fn scroll_up(&mut self, buffer: &mut Buffer) {
        if self.top > 0 {
            let height = termion::terminal_size().unwrap().1 as usize;
            if buffer.get_root_selection_line() >= self.top + height - 2 {
                buffer.select_char_up();
            }
            self.top -= 1;
        }
    }

    fn scroll_down(&mut self, buffer: &mut Buffer) {
        // why is it `- 2`?
        if self.top < buffer.len_lines() - 2 {
            if buffer.get_root_selection_line() <= self.top {
                buffer.select_char_down();
            }
            self.top += 1;
        }
    }

    fn render(&mut self, buffer: &Buffer, command: &String) {
        let (width, height) = termion::terminal_size().unwrap();

        let selection_line = buffer.get_root_selection_line();
        if selection_line > self.top + height as usize - 2 {
            self.top += 1;
        } else if selection_line < self.top {
            self.top -= 1;
        }

        if self.processed_buffer.len_chars() == 0 {
            self.processed_buffer = buffer.data.clone();
        }

        let start_line_idx = self.processed_buffer.line_to_char(self.top);
        let end_line_idx = self
            .processed_buffer
            .line_to_char(self.top + height as usize);
        let visible_content = self.processed_buffer.slice(start_line_idx..end_line_idx);

        let mut display = Display::new(
            Rect {
                height: height - 1,
                width: width,
            },
            &visible_content.to_string(),
        );

        let highlights = self.highlighter.get_highlights(
            &self.processed_buffer.to_string(),
            start_line_idx,
            end_line_idx,
        );
        for highlight in highlights {
            let (start_point, end_point) = char_idxs_to_points(
                highlight.start,
                highlight.end,
                start_line_idx,
                visible_content,
            );
            display.highlight(start_point, end_point, Some(&highlight.color), None);
        }

        for selection in &buffer.selections {
            let (start_point, end_point) = char_idxs_to_points(
                selection.start,
                selection.end,
                start_line_idx,
                visible_content,
            );
            display.highlight(
                start_point,
                end_point,
                None,
                Some(&termion::color::LightBlack.bg_str().to_string()),
            );
        }

        write!(
            self.output,
            "{}{}",
            termion::cursor::Goto(1, 1),
            display.rendered()
        )
        .unwrap();
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
