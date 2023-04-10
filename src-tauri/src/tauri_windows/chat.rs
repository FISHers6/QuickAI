
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
            window.set_focus().unwrap();
            window.show().unwrap();
        }
        None => {
            tracing::info!("not found chat window");
            let windows = tauri::WindowBuilder::new(
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
            .inner_size(440.0, 540.0)
            .center()
            .build()
            .unwrap();
        }
    }
    if let Some(question) = question {
        let _err = crate::event::trigger_question_update(handle, question, true);
    }
}

