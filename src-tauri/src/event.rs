use crate::APP;
use anyhow::Result;
use tauri::Manager;

pub fn trigger_message_update(message: String) -> Result<()> {
    APP.get().unwrap().emit_all("update-message", message)?;
    Ok(())
}