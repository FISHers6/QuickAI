use serde::{Deserialize, Serialize};
use tauri::{Manager, Size, Window, LogicalSize};
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
    crate::tauri_windows::search::show_foreground_window();

    let handle = crate::APP.get().unwrap();
    let state: tauri::State<crate::AppState> = handle.state();
    let answer_sender = get_or_init_auto_input(&state);
    answer_sender
        .send(payload.response)
        .map_err(|err| format!("{:?}", err))?;
    Ok(())
}

#[tauri::command]
pub async fn send_auto_input_value(payload: AutoInput) -> Result<(), String> {
    tracing::info!(payload =? payload);
    let handle = crate::APP.get().unwrap();
    let state: tauri::State<crate::AppState> = handle.state();
    let answer_sender = get_or_init_auto_input(&state);
    answer_sender
        .send(payload.response)
        .map_err(|err| format!("{:?}", err))?;
    Ok(())
}


fn get_or_init_auto_input<'a>(state: &'a tauri::State<crate::AppState>) -> &'a UnboundedSender<String> {
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