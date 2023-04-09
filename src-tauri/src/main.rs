// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod easy_thing;
mod event;
mod select;
mod shortcut;
mod tauri_windows;

use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use std::sync::Arc;
use tauri::AppHandle;
use tauri::Manager;
use tokio::sync::mpsc::UnboundedSender;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

use std::sync::atomic::AtomicIsize;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

pub static APP: OnceCell<AppHandle> = OnceCell::new();
pub struct AppState {
    pub selected_content: Arc<RwLock<String>>,
    pub foreground_handle: AtomicIsize,
    runtime: Runtime,
    pub auto_input_sender: OnceCell<UnboundedSender<String>>,
}

impl AppState {
    pub fn new(runtime: Runtime) -> Self {
        Self {
            selected_content: Arc::new(RwLock::new(String::new())),
            foreground_handle: AtomicIsize::new(0),
            runtime,
            auto_input_sender: OnceCell::new(),
        }
    }

    pub fn spawn_future<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.runtime.spawn(future)
    }

    pub fn spawn_task<F>(&self, task: F) -> JoinHandle<()>
    where
        F: Send + 'static + FnOnce() -> (),
    {
        self.runtime.spawn(async move { task() })
    }
}

fn main() {
    tracing_subscriber::registry().with(fmt::layer()).init();
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            tracing::info!("{}, {argv:?}, {cwd}", app.package_info().name);
        }))
        .invoke_handler(tauri::generate_handler![
            command::get_selected_content,
            command::set_size,
            command::run_auto_input,
            command::send_auto_input_value,
            command::run_quick_answer,
        ])
        .setup(|app| {
            tracing::info!(start = true);
            APP.get_or_init(|| app.handle());
            let app_handle = app.handle();
            app_handle.manage(AppState::new(
                tokio::runtime::Runtime::new().expect("build tokio runtime error"),
            ));

            // 注册全局快捷键
            let _ = shortcut::ShortcutRegister::register_shortcut(&app_handle);

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| match event {
            tauri::RunEvent::WindowEvent { label, event, .. } => {
                if label == "main" {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        tracing::info!(label = label, prevent_close = true);
                        if let Some(main_window) = app_handle.get_window(&label) {
                            let _ = main_window.hide();
                        }
                        api.prevent_close()
                    }
                }
            }
            tauri::RunEvent::ExitRequested { api, .. } => {
                tracing::info!("exit");
                api.prevent_exit();
            }
            _ => {}
        });
}
