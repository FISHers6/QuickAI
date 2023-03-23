// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod select;
mod event;
mod command;
mod shortcut;
mod windows;

use once_cell::sync::OnceCell;
use tauri::AppHandle;
use tauri::Manager;
use std::sync::Arc;
use parking_lot::RwLock;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub static APP: OnceCell<AppHandle> = OnceCell::new();
pub struct AppState {
    pub selected_content: Arc<RwLock<String>>,
}

impl AppState {
    pub fn new() -> Self{
        Self {
            selected_content: Arc::new(RwLock::new(String::new()))
        }
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);
            // windows::chatgpt_windows();
        }))
        .invoke_handler(tauri::generate_handler![
            greet, 
            command::get_selected_text,
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
