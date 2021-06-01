mod buffer;
pub mod modes;

use crate::view::terminal::Terminal;
use crate::view::traits::View;
use buffer::Buffer;
use modes::Modes;
use std::fs;
use termion::event::Key;
use termion::input::{Keys, TermRead};
use termion::AsyncReader;

/// `Application` handles the logic of the application and is responsible for managing state.
pub struct Application {
    quit: bool,
    mode: Modes,
    command: String,
    view: Terminal,
    input: Keys<AsyncReader>,
    buffer: Buffer,
}

impl Application {
    pub fn new() -> Application {
        Application {
            quit: false,
            mode: Modes::Normal,
            command: String::from(""),
            view: Terminal::new(),
            input: termion::async_stdin().keys(),
            buffer: Buffer::new(fs::read_to_string("src/application/mod.rs").unwrap()),
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
            Key::Esc => self.view.change_mode(Modes::Normal),
            Key::Char(c) => self.view.write_char(c),
            _ => {}
        }
    }

    fn change_mode(&mut self, mode: Modes) {
        self.mode = mode;
        self.view.change_mode(mode);
    }

    fn handle_normal_mode_event(&mut self, event: termion::event::Key) {
        match event {
            Key::Char('q') => self.quit = true,
            Key::Char('i') => self.change_mode(Modes::Insert),
            Key::Char(':') => self.change_mode(Modes::Command),
            Key::Ctrl('e') => self.view.scroll_down(),
            Key::Ctrl('y') => self.view.scroll_up(),
            _ => {}
        }
    }

    fn handle_command_mode_event(&mut self, event: termion::event::Key) {
        match event {
            Key::Backspace => {
                self.command.pop();
            }
            Key::Char('\n') => self.handle_command(),
            Key::Char(c) => self.command.push(c),
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
        self.view.start();
        self.view.render(&self.buffer.data, &self.command);
        self.listen();
    }

    fn listen(mut self) {
        while !self.quit {
            let input = self.input.next();

            if let Some(event) = input {
                self.handle_event(event.unwrap());
                self.view.render(&self.buffer.data, &self.command);
            }
        }
    }
}
