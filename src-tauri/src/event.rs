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
    tracing::info!("trigger changed");
    Ok(())
}

pub fn trigger_chat_question_update(handle: &AppHandle, question: String) -> Result<()> {
    handle.emit_all("change-chat-question-content", question)?;
    tracing::info!("trigger changed");
    Ok(())
}
