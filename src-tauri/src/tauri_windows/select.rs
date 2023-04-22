use std::sync::atomic::Ordering;

use crate::AppState;
use tauri::{Manager, WindowEvent, LogicalPosition, PhysicalPosition};
#[cfg(any(target_os = "macos", target_os = "windows"))]
use window_shadows::set_shadow;
use window_vibrancy;

use crate::easy_thing::foreground::PlatformForeground;
use crate::APP;
#[cfg(target_os = "macos")]
use window_vibrancy::NSVisualEffectMaterial;
pub const SELECT_WINDOWS: &str = "select_windows";
pub const SELECT_WINDOWS_WIDTH: f64 = 240.0;
pub const SELECT_WINDOWS_HEIGHT: f64 = 80.0;

pub fn build_select_windows(content: &str, window_position_x: f64, window_position_y: f64) {
    let foreground_handle = PlatformForeground::get_foreground_window();
    tracing::info!(foreground_handle = foreground_handle);
    let handle = APP.get().unwrap();
    let state: tauri::State<AppState> = handle.state();
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