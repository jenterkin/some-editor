mod application;
mod view;
mod logger;
mod highlight;

use crate::application::Application;
use crate::logger::setup_logger;

#[tokio::main]
async fn main() {
    setup_logger().unwrap();
    Application::new().start().await;
}
