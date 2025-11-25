mod app;
mod types;
mod update;
mod view;
mod api;
mod runtime;

use app::AdminApp;
use iced::{application, Theme};
use runtime::init_runtime;
use update::update;
use view::view;

fn main() -> iced::Result {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .init();
    
    // Initialize Tokio runtime for async operations
    init_runtime();
    
    // Run the application
    application("Bitcoin Admin UI", update, view)
        .theme(|_| Theme::Dark)
        .run_with(AdminApp::new)
}
