pub mod buffer;
pub mod modes;

use log::{debug};
use crate::view::terminal::Terminal;
use crate::view::traits::View;
use buffer::Buffer;
use modes::Modes;
use std::fs;
use std::io::stdin;
use termion::event::Key;
use termion::input::TermRead;
use tokio::sync::mpsc;

/// `Application` handles the logic of the application and is responsible for managing state.
pub struct Application {
    quit: bool,
    mode: Modes,
    command: String,
    view: Terminal,
    buffer: Buffer,
}

impl Application {
    pub fn new() -> Application {
        Application {
            quit: false,
            mode: Modes::Normal,
            command: String::from(""),
            view: Terminal::new(),
            buffer: Buffer::new(fs::read_to_string("src/application/mod.rs").unwrap()),
        }
    }

    fn handle_command(&mut self) {
        debug!("handling command: {}", self.command.as_str());
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
            // Selections
            Key::Char('h') => self.buffer.select_char_left(),
            Key::Char('j') => self.buffer.select_char_down(),
            Key::Char('k') => self.buffer.select_char_up(),
            Key::Char('l') => self.buffer.select_char_right(),
            // Scrolling
            Key::Ctrl('e') => self.view.scroll_down(self.buffer.len_lines()),
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

    pub async fn start(mut self) {
        self.view.start();
        self.view.render(&self.buffer, &self.command);

        let (sender, mut receiver) = mpsc::unbounded_channel();
        self.listen(sender);
        self.handle_events(&mut receiver).await;
    }

    async fn handle_events(&mut self, receiver: &mut mpsc::UnboundedReceiver<Key>) {
        while !self.quit {
            let input = receiver.recv().await;
            if let Some(event) = input {
                self.handle_event(event);
                self.view.render(&self.buffer, &self.command);
            }
        }
    }

    fn listen(&mut self, sender: mpsc::UnboundedSender<Key>) {
        let stdin = stdin();
        tokio::task::spawn_blocking(move || {
            for input in stdin.keys() {
                if let Ok(event) = input {
                    sender.send(event).unwrap();
                }
            };
        });
    }
}
