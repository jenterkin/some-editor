mod modes;

use modes::Modes;
use std::io::{stdout, BufWriter, Stdout, Write};
use termion::event::Key;
use termion::input::{Keys, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use termion::AsyncReader;

#[derive(Clone)]
struct CursorPosition {
    row: u16,
    col: u16,
}

pub struct Application {
    input: Keys<AsyncReader>,
    output: BufWriter<RawTerminal<AlternateScreen<Stdout>>>,
    last_position: CursorPosition,
    position: CursorPosition,
    quit: bool,
    mode: Modes,
    command: String,
}

impl Application {
    pub fn new() -> Application {
        Application {
            input: termion::async_stdin().keys(),
            output: BufWriter::with_capacity(
                1_048_576,
                AlternateScreen::from(stdout()).into_raw_mode().unwrap(),
            ),
            last_position: CursorPosition { row: 1, col: 1 }, // TODO(jenterkin): should probably be optional
            position: CursorPosition { row: 1, col: 1 },
            quit: false,
            mode: Modes::Insert,
            command: String::from(""),
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
            _ => self.mode = mode,
        }
    }

    fn handle_command(&mut self) {
        match self.command.as_str() {
            "q" => self.quit = true,
            _ => self.change_mode(Modes::Normal),
        }
    }

    fn handle_insert_mode_event(&mut self, event: termion::event::Key) {
        match event {
            // TODO(jenterkin): is there a more elegant way to quit?
            Key::Esc => self.change_mode(Modes::Normal),
            Key::Char('\n') => {
                self.position.col = 1;
                self.position.row = self.position.row + 1;
                self.write_char('\n');
            }
            Key::Char(c) => {
                self.position.col += 1;
                self.write_char(c);
            }
            _ => {}
        }
    }

    fn handle_normal_mode_event(&mut self, event: termion::event::Key) {
        match event {
            Key::Char('q') => self.quit = true,
            Key::Char('i') => self.change_mode(Modes::Insert),
            Key::Char(':') => self.change_mode(Modes::Command),
            _ => {}
        }
    }

    fn handle_command_mode_event(&mut self, event: termion::event::Key) {
        match event {
            Key::Backspace => {
                self.command.pop();
            }
            Key::Char('\n') => self.handle_command(),
            Key::Char(c) => {
                self.command.push(c);
                self.position.col += 1;
                self.write_char(c);
            }
            Key::Esc => self.change_mode(Modes::Normal),
            _ => {}
        }
    }

    fn handle_event(&mut self, event: termion::event::Key) {
        match self.mode {
            Modes::Insert => self.handle_insert_mode_event(event),
            Modes::Normal => self.handle_normal_mode_event(event),
            Modes::Command => self.handle_command_mode_event(event),
        }
    }

    pub fn start(mut self) {
        // Set the initial position.
        self.update_position(self.position.col, self.position.row);
        self.listen();
    }

    fn listen(mut self) {
        while !self.quit {
            let input = self.input.next();

            if let Some(event) = input {
                self.handle_event(event.unwrap());
            }

            self.render()
        }
    }

    fn write_char(&mut self, c: char) {
        write!(self.output, "{}", c,).unwrap();
        self.update_position(self.position.row, self.position.col);
    }

    fn render(&mut self) {
        self.output.flush().unwrap();
    }
}
