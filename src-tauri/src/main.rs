// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod easy_thing;
mod event;
mod select;
mod shortcut;
mod tauri_windows;
#[cfg(not(target_os="macos"))]
mod task;
mod app_config;
mod utils;

use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use tauri::WindowEvent;
use std::sync::Arc;
use tauri::AppHandle;
use tauri::Manager;
use tokio::sync::mpsc::UnboundedSender;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use std::time::Duration;
use std::sync::atomic::AtomicIsize;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

pub static APP: OnceCell<AppHandle> = OnceCell::new();
pub struct AppState {
    pub selected_content: Arc<RwLock<String>>,
    pub foreground_handle: AtomicIsize,
    runtime: Runtime,
    pub auto_input_sender: OnceCell<UnboundedSender<String>>,
    pub screen_size: (f64, f64), // (width, height)
}

impl AppState {
    pub fn new(runtime: Runtime, screen_size: (f64, f64)) -> Self {
        Self {
            selected_content: Arc::new(RwLock::new(String::new())),
            foreground_handle: AtomicIsize::new(0),
            runtime,
            auto_input_sender: OnceCell::new(),
            screen_size,
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

    pub fn spawn_delay_task<F>(&self, future: F, delay_time: Duration) -> JoinHandle<F::Output>
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.runtime.spawn(async move {
            tokio::time::sleep(delay_time).await;
            future.await
        })
    }
}
fn main() {
    tracing_subscriber::registry().with(fmt::layer()).init();
    // get screen size
    let screen_size = crate::utils::get_screen_size().unwrap_or((1920.0, 1080.0));
    tracing::info!(screen_size =? screen_size);
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            tracing::info!("{}, {argv:?}, {cwd}", app.package_info().name);
        }))
        .invoke_handler(tauri::generate_handler![
            command::get_selected_content,
            command::set_size,
            command::run_auto_input,
            command::send_auto_input_value,
            command::run_quick_answer,
            command::run_chat_mode,
            command::close_window,
            command::open_setting_window,
            command::copy_select_content,
            command::update_shortcut,
            command::update_app_config,
            command::get_selected_content_from_cache,
        ]);
    #[cfg(not(target_os="macos"))]
    let builder = builder.invoke_handler(tauri::generate_handler![
        command::hide_select_window,
        command::trigger_select_click,
    ]);
    builder.setup(move |app| {
        tracing::info!(start = true);
        APP.get_or_init(|| app.handle());
        let app_handle = app.handle();
        app_handle.manage(AppState::new(
            tokio::runtime::Runtime::new().expect("build tokio runtime error"),
            screen_size,
        ));

        // 注册全局快捷键
        let _ = shortcut::ShortcutRegister::register_shortcut(&app_handle);
        #[cfg(not(target_os="macos"))]
        task::register_task(&app_handle);
        Ok(())
    })
    .build(tauri::generate_context!())
    .expect("error while running tauri application")
    .run(|app_handle, event| match event {
        tauri::RunEvent::WindowEvent { label, event, .. } => {
            if label == crate::tauri_windows::chatgpt::CHATGPT_WINDOWS {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    tracing::info!(label = label, prevent_close = true);
                    if let Some(main_window) = app_handle.get_window(&label) {
                        let _ = main_window.hide();
                    }
                    api.prevent_close()
                }
            }else if label == crate::tauri_windows::SELECT_WINDOWS 
                || label == crate::tauri_windows::search::SEARCH_WINDOWS {
                if let WindowEvent::Focused(focused) = event {
                    tracing::info!(label = label, focused = focused);
                    if !focused {
                        if let Some(window) = app_handle.get_window(&label) {
                            let _ = window.hide();
                        }
                    }
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
