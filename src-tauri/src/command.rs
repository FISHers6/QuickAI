use serde::{Deserialize, Serialize};
use tauri::{Size, Window, LogicalSize};
use tokio::sync::mpsc::UnboundedSender;

#[tauri::command]
pub fn get_selected_content() -> Result<String, String> {
    crate::select::selected_text().map_or_else(
        |err| Err(err.to_string()),
        |message| {
            if message.is_empty() {
                Err("can't send empty question".to_string())
            } else {
                Ok(message)
            }
        },
    )
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetSizePayload {
    width: u32,
    height: u32,
}

#[tauri::command]
pub fn set_size(window: Window, payload: SetSizePayload) {
    tracing::info!(payload =? payload);
    window
    .set_size(Size::Logical(LogicalSize::new(
        payload.width as f64,
        payload.height as f64,
    )))
    .unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoInput {
    response: String,
}

#[tauri::command]
pub async fn run_auto_input(window: Window, payload: AutoInput) -> Result<(), String> {
    tracing::info!(payload =? payload);
    window.hide().map_err(|err| format!("{:?}", err))?;
    crate::easy_thing::send_auto_input_value(payload.response)?;
    Ok(())
}

#[tauri::command]
pub async fn send_auto_input_value(payload: AutoInput) -> Result<(), String> {
    tracing::info!(payload =? payload);
        crate::easy_thing::send_auto_input_value(payload.response)?;
    Ok(())
}

pub fn get_or_init_auto_input<'a>(state: &'a tauri::State<crate::AppState>) -> &'a UnboundedSender<String> {
    let answer_sender = state.auto_input_sender.get_or_init(|| {
        let (answer_sender, mut answer_receiver) = tokio::sync::mpsc::unbounded_channel::<String>();
        let _ = state.spawn_future(async move {
            let mut content = String::new();
            while let Some(answer) = answer_receiver.recv().await {
                if let Some(suffix) = answer.strip_prefix(&content) {
                    tracing::info!(split_suffix = true);
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    // crate::easy_thing::input::PlatformInput::send_content_v2(suffix);
                    if let Err(err) = crate::easy_thing::input::PlatformInput::auto_input_text_using_copy(suffix) {
                        tracing::warn!(err =? err);
                    }
                }else {
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    tracing::info!(split_suffix = false);
                    if let Err(err) = crate::easy_thing::input::PlatformInput::auto_input_text_using_copy(&answer) {
                        tracing::warn!(err =? err);
                    }
                }
                content = answer
            }
        });
        answer_sender
    });
    answer_sender
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestionPayload {
    question: String,
}

#[tauri::command]
pub async fn run_quick_answer(window: Window, payload: QuestionPayload) -> Result<(), String> {
    tracing::info!(payload =? payload);
    let question = if payload.question.is_empty() {
        None
    } else {
        Some(payload.question)
    };
    let error = std::panic::catch_unwind(|| {
        crate::tauri_windows::chatgpt::show_quick_answer_window(question, true);
    });
    if let Err(error) = error {
        tracing::warn!(panic =? error);
    } else {
        window.hide().map_err(|err| format!("{:?}", err))?;
    }
    Ok(())
}


#[tauri::command]
pub async fn run_chat_mode(window: Window, payload: QuestionPayload) -> Result<(), String> {
    tracing::info!(payload =? payload);
    let question = if payload.question.is_empty() {
        None
    } else {
        Some(payload.question)
    };
    let error = std::panic::catch_unwind(|| {
        crate::tauri_windows::chat::show_chat_windows(question);
    });
    if let Err(error) = error {
        tracing::warn!(panic =? error);
    } else {
        window.hide().map_err(|err| format!("{:?}", err))?;
    }
    Ok(())
}

#[tauri::command]
pub async fn close_window(window: Window) {
    if let Err(err) = window.hide() {
        tracing::warn!(close_window =? err)
    }
}

#[tauri::command]
pub async fn open_setting_window(_window: Window) {
    crate::tauri_windows::settings::build_setting_window();
}

#[tauri::command]
pub async fn hide_select_window(_window: Window) {
    crate::tauri_windows::select::hide_select_window();
}

#[tauri::command]
pub async fn copy_select_content(payload: String) -> Result<(), String> {
    crate::select::copy_content(payload).map_err(|err| format!("{:?}", err))
}


#[tauri::command]
pub fn update_shortcut() -> Result<(), String> {
    let handle = crate::APP.get().ok_or("can't get app handle")?;
    crate::shortcut::ShortcutRegister::register_shortcut(handle).map_err(|err| format!("register short cut error :{}", err))?;
    Ok(())
}

#[tauri::command]
pub fn update_app_config(payload: crate::app_config::AppConfig) -> Result<(), String> {
    tracing::info!(app_config =? payload);
    crate::app_config::save_app_config(&payload)
}

#[tauri::command] 
pub async fn trigger_select_click(handle: tauri::AppHandle, payload: crate::tauri_windows::select::SelectPayload) -> Result<(), String> {
    tracing::info!(select_click_payload =?payload);
    crate::tauri_windows::select::click_select(&handle, payload).map_err(|err| format!("trigger select click error {:?}", err))?;
    Ok(())
}