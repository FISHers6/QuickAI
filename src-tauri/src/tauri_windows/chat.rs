
use crate::AppState;
use tauri::Manager;

use crate::APP;
pub const CHAT_WINDOWS: &str = "chat_windows";

pub fn chat_windows() {
    show_chat_windows(None);
}

pub fn show_chat_windows(question: Option<String>) {
    let handle = APP.get().unwrap();
    let state: tauri::State<AppState> = handle.state();
    match handle.get_window(CHAT_WINDOWS) {
        Some(window) => {
            tracing::info!("has chat window");
            window.unminimize().unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();
        }
        None => {
            let window_width = 440.0;
            let window_height = 540.0;
            let pos_x = state.screen_size.0 - window_width - 30.0;
            let pos_y = (state.screen_size.1 - window_width) / 2.8;

            let _windows = tauri::WindowBuilder::new(
                handle,
                CHAT_WINDOWS,
                tauri::WindowUrl::App("chat.html".into()),
            )
            .title("会话")
            .resizable(false)
            .decorations(false)
            .transparent(true)
            .always_on_top(true)
            .maximized(false)
            .skip_taskbar(false)
            .inner_size(window_width, window_height)
            .position(pos_x, pos_y)
            .focused(true)
            .build()
            .unwrap();
        }
    }
    if let Some(question) = question {
        state.spawn_delay_task(async {
            let handle = APP.get().unwrap();
            if let Err(err) = crate::event::trigger_chat_question_update(handle, question) {
                tracing::warn!(err =? err);
            }else {
                if let Some(windows) = handle.get_window(CHAT_WINDOWS) {
                    let _ = windows.set_focus();
                }
            }
        }, std::time::Duration::from_millis(800));
    }
}

