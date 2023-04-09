use tauri::LogicalPosition;
use tauri::Manager;
use tauri::PhysicalPosition;

use crate::select;
use crate::AppState;
use crate::APP;
use anyhow::{anyhow, Result};
use mouse_position::mouse_position::Mouse;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use window_shadows::set_shadow;

const CHATGPT_WINDOWS: &'static str = "main";
const MOUSE_OFFSET_X: i32 = 20;
const MOUSE_OFFSET_Y: i32 = 0;

fn current_mouse_position_with_offset() -> Result<(i32, i32)> {
    let position = Mouse::get_mouse_position();
    match position {
        Mouse::Position { x, y } => Ok((MOUSE_OFFSET_X + x, MOUSE_OFFSET_Y + y)),
        Mouse::Error => Err(anyhow!("get mouse position failed")),
    }
}

pub fn chatgpt_windows() {
    let handle = APP.get().unwrap();
    let selected_text = select::selected_text().unwrap_or_default();
    let state: tauri::State<AppState> = handle.state();
    let mut trigger_selected_content_change_event = false;
    if !selected_text.is_empty() {
        let mut select_content_state = state.selected_content.write();
        if *select_content_state != selected_text {
            *select_content_state = selected_text;
            trigger_selected_content_change_event = true;
        }
    }
    let question = if trigger_selected_content_change_event {
        Some(state.selected_content.read().clone())
    } else {
        None
    };
    show_quick_answer_window(question, false);
    if trigger_selected_content_change_event {
        let _err = crate::event::trigger_question_update(
            handle,
            state.selected_content.read().clone(),
            true,
        );
    }
}

pub fn show_quick_answer_window(question: Option<String>, is_center: bool) {
    let handle = APP.get().unwrap();
    match handle.get_window(CHATGPT_WINDOWS) {
        Some(window) => {
            if is_center {
                window.center().unwrap();
            } else {
                if let Ok((x, y)) = current_mouse_position_with_offset() {
                    if cfg!(target_os = "macos") {
                        let _ = window.set_position(LogicalPosition::new(x as f64, y as f64));
                    } else {
                        let _ = window.set_position(PhysicalPosition::new(x as f64, y as f64));
                    }
                } else {
                    window.center().unwrap();
                }
            }

            window.unminimize().unwrap();
            window.set_focus().unwrap();
            window.show().unwrap();
        }
        None => {
            tracing::info!("not found chatgpt window");
            let builder = tauri::WindowBuilder::new(
                handle,
                CHATGPT_WINDOWS,
                tauri::WindowUrl::App("index.html".into()),
            )
            .inner_size(560.0, 600.0)
            .always_on_top(false)
            .fullscreen(false)
            .decorations(false)
            .skip_taskbar(false)
            .center();

            let window = builder.build().unwrap();
            #[cfg(any(target_os = "macos", target_os = "windows"))]
            {
                set_shadow(&window, true).unwrap_or_default();
            }
            window.unminimize().unwrap();
            window.set_focus().unwrap();
            window.show().unwrap();
        }
    }
    if let Some(question) = question {
        let _err = crate::event::trigger_question_update(handle, question, true);
    }
}