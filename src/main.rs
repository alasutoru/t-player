#![allow(non_snake_case)]

mod app;
mod components;
mod server;

use dioxus::prelude::*;
use dioxus::desktop::{Config, WindowBuilder, LogicalSize};
use crate::app::App;

use std::sync::OnceLock;

pub static SERVER_PORT: OnceLock<u16> = OnceLock::new();

fn main() {
    env_logger::init();
    
    // Start Local Server
    let rt = tokio::runtime::Runtime::new().unwrap();
    let port = rt.block_on(server::start_server());
    let _ = SERVER_PORT.set(port);

    // Keep runtime alive in a separate thread because Dioxus takes over the main thread
    std::thread::spawn(move || {
        rt.block_on(async {
            loop { tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await; }
        });
    });
    
    let config = Config::default()
        .with_window(WindowBuilder::new()
            .with_title("T-Player | Binary Media Center")
            .with_inner_size(LogicalSize::new(1000.0, 700.0))
            .with_resizable(true)
            .with_transparent(false)); 

    LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(App);
}
