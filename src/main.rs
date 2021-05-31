pub mod application;
pub mod view;

use crate::application::Application;

fn main() {
    Application::new().start();
}
