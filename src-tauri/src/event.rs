
use anyhow::Result;
use tauri::{Manager, AppHandle};

pub fn trigger_message_update(handle: &AppHandle, selected: String) -> Result<()> {
    handle.emit_all("change-selected-content", selected)?;
    Ok(())
}
