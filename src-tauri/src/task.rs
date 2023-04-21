use tauri::AppHandle;
use tauri::Manager;
use crate::AppState;

pub fn register_task(handle: &AppHandle) {
    let _state: tauri::State<AppState> = handle.state();
}