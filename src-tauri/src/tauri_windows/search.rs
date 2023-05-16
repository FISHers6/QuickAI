use std::sync::atomic::Ordering;

use crate::AppState;
use tauri::{Manager, WindowEvent};
#[cfg(any(target_os = "macos", target_os = "windows"))]
use window_shadows::set_shadow;
use window_vibrancy;

use crate::easy_thing::foreground::PlatformForeground;
use crate::APP;
#[cfg(target_os = "macos")]
use window_vibrancy::NSVisualEffectMaterial;
pub const SEARCH_WINDOWS: &str = "search_windows";

pub fn search_windows() {
    let foreground_handle = {
        #[cfg(target_os = "windows")]
        let hwnd = PlatformForeground::get_foreground_window();
        #[cfg(target_os = "macos")]
        let hwnd = PlatformForeground::get_foreground_window()
            .map_err(|err| {
                tracing::warn!(err =?err);
                err
            })
            .unwrap_or_default();
        tracing::info!(hwnd = hwnd);
        hwnd
    };
    tracing::info!(foreground_handle = foreground_handle);
    let handle = APP.get().unwrap();
    let state: tauri::State<AppState> = handle.state();
    if foreground_handle != 0 {
        state
            .foreground_handle
            .store(foreground_handle, Ordering::SeqCst);
    }
    match handle.get_window(SEARCH_WINDOWS) {
        Some(window) => {
            tracing::info!("has search window");
            window.unminimize().unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();
        }
        None => {
            tracing::info!("not found search window");
            let window_width = 400.0;
            let window_height = 60.0;
            let offset_y = 300.0; // 向上偏移 300 个像素
            let _pos_x = (state.screen_size.0 - window_width) / 2.0;
            let _pos_y = ((state.screen_size.1 - window_height) / 2.0 - offset_y)
                .min(state.screen_size.1 / 2.0);

            let windows = tauri::WindowBuilder::new(
                handle,
                SEARCH_WINDOWS,
                tauri::WindowUrl::App("src/search.html".into()),
            )
            .title("Search")
            .resizable(false)
            .decorations(false)
            .transparent(true)
            .always_on_top(true)
            .maximized(false)
            .skip_taskbar(true)
            .inner_size(window_width, window_height)
            .focused(true)
            .center()
            // .position(pos_x, pos_y)
            .build()
            .expect("build windows error not happened");
            // 仅在 macOS 下执行
            #[allow(deprecated)]
            #[cfg(target_os = "macos")]
            let _ = window_vibrancy::apply_vibrancy(
                &windows,
                NSVisualEffectMaterial::UltraDark,
                None,
                None,
            )
            .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            // 仅在 windows 下执行
            #[cfg(target_os = "windows")]
            window_vibrancy::apply_blur(&windows, Some((2, 2, 2, 190)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            #[cfg(any(target_os = "macos", target_os = "windows"))]
            {
                set_shadow(&windows, true).unwrap_or_default();
            }
            windows.on_window_event(hide_window_when_lose_focused);
            let _ = windows.start_dragging();
        }
    }
}

pub fn show_foreground_window() {
    let handle = APP.get().unwrap();
    let state: tauri::State<AppState> = handle.state();
    let foreground_handle = state.foreground_handle.load(Ordering::SeqCst);
    if foreground_handle != 0 {
        let _ = PlatformForeground::set_foreground_window(foreground_handle);
    }
}

fn hide_window_when_lose_focused(event: &WindowEvent) {
    if let WindowEvent::Focused(focused) = event {
        if !focused {
            let handle = APP.get().unwrap();
            if let Some(window) = handle.get_window(SEARCH_WINDOWS) {
                let _ = window.hide();
            }
        }
    }
}
