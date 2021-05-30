mod modes;

use modes::Modes;
use std::io::{stdout, BufWriter, Stdout, Write};
use termion::event::Key;
use termion::input::{Keys, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use termion::AsyncReader;

struct CursorPosition {
    row: u16,
    col: u16,
}

pub struct Application {
    input: Keys<AsyncReader>,
    output: BufWriter<RawTerminal<AlternateScreen<Stdout>>>,
    position: CursorPosition,
    quit: bool,
    mode: Modes,
}

impl Application {
    pub fn new() -> Application {
        Application {
            input: termion::async_stdin().keys(),
            output: BufWriter::with_capacity(
                1_048_576,
                AlternateScreen::from(stdout()).into_raw_mode().unwrap(),
            ),
            position: CursorPosition { row: 1, col: 1 },
            quit: false,
            mode: Modes::Insert,
        }
    }

    fn handle_insert_mode_event(&mut self, event: termion::event::Key) {
        match event {
            // TODO(jenterkin): is there a more elegant way to quit?
            Key::Esc => self.mode = Modes::Normal,
            Key::Char('\n') => {
                self.position.row = 1;
                self.position.col = self.position.col + 1;
                self.write_char('\n');
            }
            Key::Char(c) => {
                self.position.row += 1;
                self.write_char(c);
            }
            _ => {}
        }
    }

    fn handle_normal_mode_event(&mut self, event: termion::event::Key) {
        match event {
            Key::Char('q') => self.quit = true,
            _ => {}
        }
    }

    fn handle_event(&mut self, event: termion::event::Key) {
        match self.mode {
            Modes::Insert => self.handle_insert_mode_event(event),
            Modes::Normal => self.handle_normal_mode_event(event),
            _ => {}
        }
    }

    pub fn start(mut self) {
        // Set the initial position.
        write!(
            self.output,
            "{}",
            termion::cursor::Goto(self.position.row, self.position.col)
        )
        .unwrap();

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
        write!(
            self.output,
            "{}{}",
            c,
            termion::cursor::Goto(self.position.row, self.position.col)
        )
        .unwrap();
    }

    fn render(&mut self) {
        self.output.flush().unwrap();
    }
}
