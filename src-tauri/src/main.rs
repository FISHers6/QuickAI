// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_config;
mod command;
mod easy_thing;
mod event;
mod select;
mod shortcut;
#[cfg(not(target_os = "macos"))]
mod task;
mod tauri_windows;
mod utils;
mod tray;

use app_config::AppConfig;
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use tray::handle_click_system_tray;
use tray::system_tray;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicIsize;
use std::sync::Arc;
use std::time::Duration;
use tauri::AppHandle;
use tauri::Manager;
use tauri::WindowEvent;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::tauri_windows::chatgpt::show_quick_answer_window;

pub static APP: OnceCell<AppHandle> = OnceCell::new();
pub struct AppState {
    pub selected_content: Arc<RwLock<String>>,
    pub foreground_handle: AtomicIsize,
    runtime: Runtime,
    pub auto_input_sender: OnceCell<UnboundedSender<String>>,
    pub screen_size: (f64, f64), // (width, height)
    pub enable_select: AtomicBool,
}

impl AppState {
    pub fn new(app_config: &AppConfig, runtime: Runtime, screen_size: (f64, f64)) -> Self {
        Self {
            selected_content: Arc::new(RwLock::new(String::new())),
            foreground_handle: AtomicIsize::new(0),
            runtime,
            auto_input_sender: OnceCell::new(),
            screen_size,
            enable_select: AtomicBool::new(app_config.enable_select.unwrap_or(true)),
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

    #[allow(unused_mut)]
    let mut context = tauri::generate_context!();
    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default().plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
        show_quick_answer_window(app, None, true);
    }));

    let app_config = crate::app_config::get_app_config().unwrap_or_default();

    #[cfg(not(dev))]
    {
        use tauri::{utils::config::AppUrl, WindowUrl};
        let port = if let Ok(Some(port)) = crate::app_config::get_local_server_port() {
            port
        }else {
            let port = portpicker::pick_unused_port().expect("Failed to pick unused port");
            if let Err(err) = crate::app_config::save_local_server_port(port) {
                tracing::error!(port_save_error=?err);
            }
            port
        };
        let url = format!("http://localhost:{}", port).parse().unwrap();
        let window_url = WindowUrl::External(url);
        // rewrite the config so the IPC is enabled on this URL
        context.config_mut().build.dist_dir = AppUrl::Url(window_url.clone());
        context.config_mut().build.dev_path = AppUrl::Url(window_url);

        builder = builder.plugin(tauri_plugin_localhost::Builder::new(port).build());
    }

    #[cfg(target_os = "macos")]
    let builder = builder.invoke_handler(tauri::generate_handler![
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
    #[cfg(not(target_os = "macos"))]
    let builder = builder.invoke_handler(tauri::generate_handler![
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
        command::hide_select_window,
        command::trigger_select_click,
    ]);
    builder
        .setup(move |app| {
            tracing::info!(start = true);
            APP.get_or_init(|| app.handle());
            let app_handle = app.handle();
            app_handle.manage(AppState::new(
                &app_config,
                tokio::runtime::Runtime::new().expect("build tokio runtime error"),
                screen_size,
            ));

            // 注册全局快捷键
            let _ = shortcut::ShortcutRegister::register_shortcut(&app_handle);
            #[cfg(not(target_os = "macos"))]
            task::register_task(&app_handle);
            Ok(())
        })
        .system_tray(system_tray())
        .on_system_tray_event(handle_click_system_tray)
        .build(context)
        .expect("error while running tauri application")
        .run(|app_handle, event| match event {
            tauri::RunEvent::WindowEvent { label, event, .. } => {
                if label == crate::tauri_windows::SELECT_WINDOWS
                    || label == crate::tauri_windows::search::SEARCH_WINDOWS
                {
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
            _ => {}
        });
}
