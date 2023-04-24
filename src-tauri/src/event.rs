use anyhow::Result;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuestionPayload {
    question: String,
    trigger: bool,
}

pub fn trigger_question_update(handle: &AppHandle, question: String, trigger: bool) -> Result<()> {
    handle.emit_all(
        "change-question-content",
        QuestionPayload { question, trigger },
    )?;
    tracing::info!("trigger change-question-content");
    Ok(())
}

pub fn trigger_chat_question_update(handle: &AppHandle, question: String) -> Result<()> {
    handle.emit_all("change-chat-question-content", question)?;
    tracing::info!("trigger change-chat-question-content");
    Ok(())
}


pub fn trigger_selected_content_update(handle: &AppHandle, selected: String) -> Result<()> {
    handle.emit_all(
        "change-select-content",
        selected,
    )?;
    tracing::info!("trigger change-question-content");
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatPayload {
    question: String,
    prompt: String,
}

pub fn trigger_send_chat_api(handle: &AppHandle, question: String, prompt: String) -> Result<()> {
    handle.emit_all(
        "trigger-send-chat-api",
        ChatPayload { question, prompt },
    )?;
    tracing::info!("trigger-send-chat-api");
    Ok(())
}