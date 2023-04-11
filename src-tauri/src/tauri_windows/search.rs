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
    let foreground_handle = PlatformForeground::get_foreground_window();
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
            window.set_focus().unwrap();
            window.show().unwrap();
        }
        None => {
            tracing::info!("not found search window");
            let windows = tauri::WindowBuilder::new(
                handle,
                SEARCH_WINDOWS,
                tauri::WindowUrl::App("search.html".into()),
            )
            .title("Search")
            .resizable(false)
            .decorations(false)
            .transparent(true)
            .always_on_top(true)
            .maximized(false)
            .skip_taskbar(true)
            .inner_size(400.0, 199.0)
            .focused(true)
            .center()
            .build()
            .unwrap();
            // 仅在 macOS 下执行
            #[cfg(target_os = "macos")]
            window_vibrancy::apply_vibrancy(&windows, NSVisualEffectMaterial::FullScreenUI)
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
        }
    }

    // let content = "public class Main {\npublic static void main(String[] args) {\nSystem.out.println(\"Hello, world!\");\n}\n}";
    // PlatformForeground::set_foreground_window(foreground_handle);
    // PlatformInput::send_content(content);
}

pub fn show_foreground_window() {
    let handle = APP.get().unwrap();
    let state: tauri::State<AppState> = handle.state();
    let foreground_handle = state.foreground_handle.load(Ordering::SeqCst);
    PlatformForeground::set_foreground_window(foreground_handle);
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