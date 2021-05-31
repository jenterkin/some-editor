pub mod modes;

use crate::view::terminal::Terminal;
use crate::view::traits::View;
use modes::Modes;
use std::fs;
use termion::event::Key;
use termion::input::{Keys, TermRead};
use termion::AsyncReader;

pub struct Application {
    quit: bool,
    mode: Modes,
    command: String,
    view: Terminal,
    input: Keys<AsyncReader>,
}

impl Application {
    pub fn new() -> Application {
        Application {
            quit: false,
            mode: Modes::Normal,
            command: String::from(""),
            view: Terminal::new(fs::read_to_string("src/application/mod.rs").unwrap()),
            input: termion::async_stdin().keys(),
        }
    }

    fn handle_command(&mut self) {
        match self.command.as_str() {
            "q" => self.quit = true,
            _ => self.view.change_mode(Modes::Normal),
        }
    }

    fn handle_insert_mode_event(&mut self, event: termion::event::Key) {
        match event {
            Key::Esc => self.view.change_mode(Modes::Normal),
            Key::Char(c) => self.view.write_char(c),
            _ => {}
        }
    }

    fn handle_normal_mode_event(&mut self, event: termion::event::Key) {
        match event {
            Key::Char('q') => self.quit = true,
            Key::Char('i') => self.view.change_mode(Modes::Insert),
            Key::Char(':') => self.view.change_mode(Modes::Command),
            Key::Ctrl('e') => self.view.scroll_down(),
            Key::Ctrl('y') => self.view.scroll_up(),
            _ => {}
        }
    }

    fn handle_command_mode_event(&mut self, event: termion::event::Key) {
        match event {
            Key::Backspace => self.view.command_pop(),
            Key::Char('\n') => self.handle_command(),
            Key::Char(c) => self.view.command_push(c),
            Key::Esc => self.view.change_mode(Modes::Normal),
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
        self.listen();
    }

    fn listen(mut self) {
        while !self.quit {
            let input = self.input.next();

            if let Some(event) = input {
                self.handle_event(event.unwrap());
            }

            self.view.render()
        }
    }
}
