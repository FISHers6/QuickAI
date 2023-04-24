
use crate::AppState;
use tauri::Manager;

use crate::APP;
pub const SETTING_WINDOWS: &str = "setting_windows";

pub fn build_setting_window() {
    let handle = APP.get().unwrap();
    let _state: tauri::State<AppState> = handle.state();
    match handle.get_window(SETTING_WINDOWS) {
        Some(window) => {
            window.unminimize().unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();
        }
        None => {
            let _windows = tauri::WindowBuilder::new(
                handle,
                SETTING_WINDOWS,
                tauri::WindowUrl::App("settings.html".into()),
            )
            .title("设置")
            .resizable(false)
            .decorations(true)
            .transparent(false)
            .always_on_top(false)
            .maximized(false)
            .skip_taskbar(false)
            .inner_size(490.0, 450.0)
            .center()
            .focused(true)
            .build()
            .unwrap();
        }
    }
}

