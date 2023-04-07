// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod event;
mod select;
mod shortcut;
mod tauri_windows;
mod easy_thing;

use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use std::sync::Arc;
use tauri::AppHandle;
use tauri::Manager;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
use std::sync::atomic::AtomicIsize;

pub static APP: OnceCell<AppHandle> = OnceCell::new();
pub struct AppState {
    pub selected_content: Arc<RwLock<String>>,
    pub foreground_handle: AtomicIsize,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            selected_content: Arc::new(RwLock::new(String::new())),
            foreground_handle: AtomicIsize::new(0),
        }
    }
}

fn main() {
    tracing_subscriber::registry()
    .with(fmt::layer())
    .init();
    tracing::info!(start = true);
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);
        }))
        .invoke_handler(tauri::generate_handler![
            greet,
            command::get_selected_content,
            command::set_size,
        ])
        .setup(|app| {
            APP.get_or_init(|| app.handle());
            let app_handle = app.handle();
            app_handle.manage(AppState::new());
            
            // 注册全局快捷键
            let _ = shortcut::ShortcutRegister::register_shortcut(&app_handle);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
