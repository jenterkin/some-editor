use crate::application::buffer::Buffer;
use crate::application::modes::Modes;

pub trait View {
    fn start(&mut self);
    fn change_mode(&mut self, mode: Modes);
    fn write_char(&mut self, char: char);
    fn scroll_up(&mut self, buffer: &mut Buffer);
    fn scroll_down(&mut self, buffer: &mut Buffer);
    fn render(&mut self, data: &Buffer, command: &String);
}
