use tauri::{Window, Size, PhysicalSize};

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

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct SetSizePayload {
  width: u32,
  height: u32,
}


#[tauri::command]
pub fn set_size(window: Window, payload: SetSizePayload) {
  tracing::info!(payload =? payload);
  window.set_size(Size::Physical(PhysicalSize::new(payload.width, payload.height))).unwrap();
}