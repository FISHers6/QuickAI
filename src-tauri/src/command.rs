


#[tauri::command]
pub fn get_selected_text() -> Option<String> {
    crate::select::selected_text().ok().and_then(|message| if message.is_empty() {
        None
    }else {
        Some(message)
    })
}