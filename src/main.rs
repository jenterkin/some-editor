extern crate termion;

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

struct Application {
    input: Keys<AsyncReader>,
    output: BufWriter<RawTerminal<AlternateScreen<Stdout>>>,
    position: CursorPosition,
    quit: bool,
}

impl Application {
    fn new() -> Application {
        Application {
            input: termion::async_stdin().keys(),
            output: BufWriter::with_capacity(
                1_048_576,
                AlternateScreen::from(stdout()).into_raw_mode().unwrap(),
            ),
            position: CursorPosition { row: 1, col: 1 },
            quit: false,
        }
    }

    fn handle_event(&mut self, event: termion::event::Key) {
        match event {
            // TODO(jenterkin): is there a more elegant way to quit?
            Key::Char('q') => self.quit = true,
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

    fn listen(mut self) {
        // Set the initial position. This should probably go somewhere else.
        write!(
            self.output,
            "{}",
            termion::cursor::Goto(self.position.row, self.position.col)
        )
        .unwrap();

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

fn main() {
    Application::new().listen();
}
