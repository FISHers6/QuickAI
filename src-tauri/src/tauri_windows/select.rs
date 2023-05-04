use std::sync::atomic::Ordering;
use crate::AppState;
use tauri::{Manager, WindowEvent, LogicalPosition, PhysicalPosition, AppHandle};
use crate::easy_thing::foreground::PlatformForeground;
use serde::{Deserialize, Serialize};
use crate::APP;
pub use super::SELECT_WINDOWS;
pub const SELECT_WINDOWS_WIDTH: f64 = 320.0;
pub const SELECT_WINDOWS_HEIGHT: f64 = 80.0;

pub fn build_select_windows(handle: &AppHandle, content: &str, window_position_x: f64, window_position_y: f64) {
    let foreground_handle = PlatformForeground::get_foreground_window().unwrap_or_default();
    tracing::info!(foreground_handle = foreground_handle);
    let state: tauri::State<AppState> = handle.state();
    let _selected = content.to_string();
    if foreground_handle != 0 {
        state
            .foreground_handle
            .store(foreground_handle, Ordering::SeqCst);
    }
    match handle.get_window(SELECT_WINDOWS) {
        Some(window) => {
            tracing::info!("has select window");
            window.unminimize().unwrap();
            if cfg!(target_os = "macos") {
                let _ = window.set_position(LogicalPosition::new(window_position_x, window_position_y));
            } else {
                let _ = window.set_position(PhysicalPosition::new(window_position_x, window_position_y));
            }
            window.show().unwrap();
            window.set_focus().unwrap();
        }
        None => {
            tracing::info!("not found select window");
            let windows = tauri::WindowBuilder::new(
                handle,
                SELECT_WINDOWS,
                tauri::WindowUrl::App("select.html".into()),
            )
            .title("select")
            .resizable(false)
            .decorations(false)
            .transparent(true)
            .always_on_top(true)
            .maximized(false)
            .skip_taskbar(true)
            .position(window_position_x as f64, window_position_y as f64)
            .inner_size(SELECT_WINDOWS_WIDTH, SELECT_WINDOWS_HEIGHT)
            .focused(true)
            .build()
            .expect("build windows error not happened");
            windows.on_window_event(hide_window_when_lose_focused);
        }
    }
}

fn hide_window_when_lose_focused(event: &WindowEvent) {
    if let WindowEvent::Focused(focused) = event {
        if !focused {
            let handle = APP.get().unwrap();
            if let Some(window) = handle.get_window(SELECT_WINDOWS) {
                let _ = window.hide();
            }
        }
    }
}

pub fn hide_select_window() {
    let handle = APP.get().unwrap();
    if let Some(window) = handle.get_window(SELECT_WINDOWS) {
       let _ = window.hide();
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectPayload {
    pub label: String,
    pub prompt: String,
    pub selected: String,
}

pub fn click_select(handle: &tauri::AppHandle, payload: SelectPayload) -> anyhow::Result<()> {
    let app_config = crate::app_config::get_app_config().unwrap_or_default();
    let mode = app_config.mode.unwrap_or("快捷提问".to_string());
    tracing::info!(mode = mode);
    match mode.as_str() {
        "快捷提问" => {
            crate::tauri_windows::chatgpt::show_quick_answer_window(handle, Some(format!("{}: {}", payload.prompt ,payload.selected)), true);
        },
        "对话模式" => {
            crate::tauri_windows::chat::show_chat_windows(Some(format!("{}: {}", payload.prompt ,payload.selected)));
        },
        "自动输入" => {
            crate::event::trigger_send_chat_api(handle, payload.selected, payload.prompt)?;
        },
        _ => {

        }
    }
    Ok(())
}