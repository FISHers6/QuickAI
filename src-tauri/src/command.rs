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
