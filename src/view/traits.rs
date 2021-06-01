use crate::application::modes::Modes;
use ropey::Rope;

pub trait View {
    fn start(&mut self);
    fn command_pop(&mut self);
    fn command_push(&mut self, char: char);
    fn change_mode(&mut self, mode: Modes);
    fn write_char(&mut self, char: char);
    fn scroll_up(&mut self);
    fn scroll_down(&mut self);
    fn render(&mut self, data: &Rope);
}
