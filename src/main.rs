mod application;
mod view;
mod logger;

use crate::application::Application;
use crate::logger::setup_logger;

#[tokio::main]
async fn main() {
    setup_logger().unwrap();
    Application::new().start().await;
}
